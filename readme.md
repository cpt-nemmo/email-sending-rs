# Usage

1. Refactor code to manage your account from which 
you want to send emails. You can do it by: 1) setting up 2-step auth in your g00gle acc; 2) generate password from this link: ***https://rb.gy/k17tlh***. 

2. Paste it in the following code below: 

```Rust
let creds = Credentials::new(
    String::from("<here your email address>"),
    String::from("<here your password>")
);
```

3. Run code `cargo run`. If everything is right you will see successful status message.