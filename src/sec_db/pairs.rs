    // // Generate RSA keys
    // let mut recipients = vec![];
    // for name in &["alice", "bob", "charlie"] {
    //     let priv_key = RsaPrivateKey::new(&mut OsRng, 2048)?;
    //     let pub_key = RsaPublicKey::from(&priv_key);
    //     recipients.push((name.to_string(), priv_key, pub_key));
    // }

    // let public_keys: Vec<(String, RsaPublicKey)> = recipients
    //     .iter()
    //     .map(|(name, _, pubkey)| (name.clone(), pubkey.clone()))
    //     .collect();

    // let message = b"Secret message for all recipients!";
    // let (ciphertext, nonce, encrypted_keys) = IKeys::encrypt(message, &public_keys)?;

    // println!("Encrypted ciphertext (base64): {}", STANDARD.encode(&ciphertext));
    // println!("Nonce (base64): {}", STANDARD.encode(&nonce));

    // let (name, private_key, _) = &recipients[2]; // e.g., Bob
    // let encrypted_key = &encrypted_keys[name];
    // let decrypted = IKeys::decrypt(encrypted_key, &ciphertext, &nonce, private_key)?;

    // println!("{} decrypted message: {}", name, String::from_utf8_lossy(&decrypted));
    // Ok(())

    // Config::read();
