use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use sha2::{Digest, Sha256};
use std::ffi::c_void;
use std::fs;
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Security::Cryptography::*;


pub(crate) fn sign_json_with_tpm(//visible only inside this crate
    //json_path: &str,
    json_bytes: &[u8],
    cert_thumbprint_hex: &str,
    out_sig_b64_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Read payload and hash
    //let payload = fs::read(json_path)?;
    let mut hasher = Sha256::new();
    //hasher.update(&payload);
    hasher.update(json_bytes);
    let digest = hasher.finalize();

    // Open CurrentUser\\My store
    let store_name = wide("MY");
    let h_store = unsafe { CertOpenSystemStoreW(0, store_name.as_ptr()) };
    if h_store.is_null() {
        return Err("Failed to open CurrentUser\\\\My certificate store".into());
    }

    // Thumbprint bytes
    let thumb_bytes = hex_to_bytes(cert_thumbprint_hex).ok_or("Invalid thumbprint hex")?;

    // Find cert by thumbprint
    let mut found_ctx: *const CERT_CONTEXT = std::ptr::null();
    let mut ctx: *const CERT_CONTEXT = std::ptr::null();
    loop {
        ctx = unsafe { CertEnumCertificatesInStore(h_store, ctx) };
        if ctx.is_null() { break; }

        let mut needed: u32 = 0;
        let ok = unsafe { CertGetCertificateContextProperty(ctx, CERT_HASH_PROP_ID, std::ptr::null_mut(), &mut needed) };
        if ok == 0 { continue; }
        if needed > 0 {
            let mut buf: Vec<u8> = vec![0u8; needed as usize];
            let ok2 = unsafe { CertGetCertificateContextProperty(ctx, CERT_HASH_PROP_ID, buf.as_mut_ptr() as *mut c_void, &mut needed) };
            if ok2 != 0 && needed as usize == buf.len() {
                if buf.len() == thumb_bytes.len() && buf[..] == thumb_bytes[..] {
                    found_ctx = ctx;
                    break;
                }
            }
        }
    }

    if found_ctx.is_null() {
        unsafe { CertCloseStore(h_store, 0); };
        return Err("Certificate with given thumbprint not found".into());
    }

    // Acquire NCRYPT key handle
    let mut hprov_or_key: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE = 0;
    let mut key_spec: u32 = 0;
    let mut must_free: BOOL = 0;
    let ok = unsafe { CryptAcquireCertificatePrivateKey(found_ctx, CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG | CRYPT_ACQUIRE_CACHE_FLAG, std::ptr::null_mut(), &mut hprov_or_key, &mut key_spec, &mut must_free) };
    if ok == 0 {
        unsafe { CertFreeCertificateContext(found_ctx); CertCloseStore(h_store, 0); }
        return Err("CryptAcquireCertificatePrivateKey failed".into());
    }
    let ph_key: NCRYPT_KEY_HANDLE = hprov_or_key as usize;

    // Sign with PKCS#1 v1.5 + SHA-256
    let mut pad_info = BCRYPT_PKCS1_PADDING_INFO { pszAlgId: BCRYPT_SHA256_ALGORITHM };
    let mut needed_len: u32 = 0;
    let status = unsafe { NCryptSignHash(ph_key, &mut pad_info as *mut _ as *mut c_void, digest.as_ptr(), digest.len() as u32, std::ptr::null_mut(), 0, &mut needed_len, BCRYPT_PAD_PKCS1) };
    if status != 0 {
        unsafe { NCryptFreeObject(ph_key); CertFreeCertificateContext(found_ctx); CertCloseStore(h_store, 0); }
        return Err(format!("NCryptSignHash (query) failed: 0x{:X}", status).into());
    }

    let mut sig = vec![0u8; needed_len as usize];
    let mut written_len: u32 = 0;
    let status2 = unsafe { NCryptSignHash(ph_key, &mut pad_info as *mut _ as *mut c_void, digest.as_ptr(), digest.len() as u32, sig.as_mut_ptr(), sig.len() as u32, &mut written_len, BCRYPT_PAD_PKCS1) };
    if status2 != 0 {
        unsafe { NCryptFreeObject(ph_key); CertFreeCertificateContext(found_ctx); CertCloseStore(h_store, 0); }
        return Err(format!("NCryptSignHash (sign) failed: 0x{:X}", status2).into());
    }
    sig.truncate(written_len as usize);

    // Write base64 signature
    let sig_b64 = BASE64.encode(&sig);
    fs::write(out_sig_b64_path, &sig_b64)?;

    // Cleanup
    unsafe {
        if must_free != 0 { let _ = NCryptFreeObject(ph_key); }
        CertFreeCertificateContext(found_ctx);
        let _ = CertCloseStore(h_store, 0);
    }

    Ok(sig_b64)
}

fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {
    let s = hex.trim().replace(" ", "").replace(":", "").to_lowercase();
    if s.len() % 2 != 0 { return None; }
    let mut out = Vec::with_capacity(s.len()/2);
    for i in 0..(s.len()/2) {
        let b = u8::from_str_radix(&s[i*2..i*2+2], 16).ok()?;
        out.push(b);
    }
    Some(out)
}

fn wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}