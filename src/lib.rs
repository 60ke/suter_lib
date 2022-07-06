#![crate_type = "cdylib"]
use std::fs;
use rand_core::OsRng;
use std::ffi::CStr;
use libc::c_char;
use suter_proofs::confidential::ConfidentialTransaction;
use suter_proofs::confidential::Transaction;
use suter_proofs::{Amount, PublicKey, SecretKey,EncryptedBalance};
use bincode;



fn read_pk(key:*const c_char)->PublicKey{
    let key = read_str(key);
    let key_bytes = fs::read_to_string(key).unwrap();
    let pk = PublicKey::from_bytes(&hex::decode(&key_bytes).unwrap()).unwrap();
    return pk;
}

fn read_sk(key:*const c_char)->SecretKey{
    let key = read_str(key);
    let key_bytes = fs::read_to_string(key).unwrap();
    let sk = SecretKey::from_bytes(&hex::decode(&key_bytes).unwrap()).unwrap();
    return sk;
}

fn read_str(value:*const c_char)-> &'static str{
    let value_ptr:&CStr = unsafe { CStr::from_ptr(value) };
    let value = value_ptr.to_str().unwrap();
    return value;
}

fn read_num(value:*const c_char)->u64{
    let value_ptr:&CStr = unsafe { CStr::from_ptr(value) };
    let value = value_ptr.to_str().unwrap();
    let num = value.parse::<u64>().expect("加密数值仅支持uint64");
    return num;  
}


fn read_ctx(value:*const c_char)->EncryptedBalance{
    let value_ptr:&CStr = unsafe { CStr::from_ptr(value) };
    let value = value_ptr.to_str().unwrap();
    let ctx:EncryptedBalance = bincode::deserialize(&hex::decode(value).unwrap()).unwrap();
    return ctx;

}


#[no_mangle]
fn generate_keys(pk_name:*const c_char,sk_name:*const c_char){
    let mut csrng = OsRng;
    let sk = SecretKey::generate_with(&mut csrng);
    let sk_name = read_str(sk_name);
    println!("生成私钥: {}",sk_name);
    fs::write(sk_name, hex::encode(sk.to_bytes())).unwrap();
    
    let pk = sk.to_public();
    let pk_name = read_str(pk_name);
    println!("生成公钥: {}",pk_name);    
    fs::write(pk_name, hex::encode(pk.to_bytes())).unwrap();
    
}

#[no_mangle]
fn encrypt_with_pubkey(key:*const c_char,value:*const c_char,out: *const c_char){

    
    let pk = read_pk(key);
    
    let num = read_num(value);

    let ctx = num.encrypt_with(pk);
    let ctx_str = hex::encode(bincode::serialize(&ctx).unwrap());

    let out = read_str(out);
    fs::write(out, &ctx_str).unwrap();

    // let bytes: Vec<u8> = ctx_str.into_bytes();
    // let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();
    // c_chars.push(0);
    // out = c_chars.as_mut_ptr();
    // return ctx_str
}

#[no_mangle]
fn decrypt_with_prikey(key:*const c_char,value:*const c_char)->u64{
    let sk = read_sk(key);
    
    let ctx = read_ctx(value);
    let num = u64::try_decrypt_from(&sk, ctx).unwrap();
    num
}

#[no_mangle]
fn gen_tx(balance:*const c_char,pk:*const c_char,sk:*const c_char,to_pk:*const c_char,to_value:*const c_char,tx_name:*const c_char){

    let sender_balance = read_ctx(balance);
    // get sender pk
    let sender_pk = read_pk(pk);
    // get sender sk
    let sender_sk = read_sk(sk);
    // get receiver pk
    let receiver_pk = read_pk(to_pk);

    let num = read_num(to_value);
    

    let mut transfers = Vec::new();
    transfers.push((receiver_pk,num));

    println!("创建交易:");

    let transaction = Transaction::<u64>::create_transaction(
        &sender_balance,
        &transfers,
        None,
        sender_pk,
        &sender_sk,
    )
    .expect("Should be able to create transaction");
    
    let tx = transaction.to_bytes().unwrap();
    let tx_name = read_str(tx_name);
    fs::write(tx_name, hex::encode(tx)).unwrap();
    println!("交易创建成功，保存为:{}",tx_name);
}

#[no_mangle]
fn verify_tx(tx:*const c_char)->i32{
    let tx_name = read_str(tx);
    let tx_bytes = hex::decode(fs::read(tx_name).unwrap()).unwrap();
    let tx:Transaction::<u64> = Transaction::from_bytes(&tx_bytes).unwrap();
    if tx.verify_transaction().is_ok(){
        return 0;
    }
    return 1;
}





#[test]
fn test_gen_tx(){
    let sender_balance:EncryptedBalance = bincode::deserialize(&hex::decode(fs::read("/Users/k/Desktop/dev/suter_lib/nodejs/ctx.txt").unwrap()).unwrap()).unwrap();

    let sender_pk = PublicKey::from_bytes(&hex::decode(fs::read("/Users/k/Desktop/dev/suter_lib/nodejs/sender_pub.key").unwrap()).unwrap()).unwrap();

    let sender_sk = SecretKey::from_bytes(&hex::decode(fs::read("/Users/k/Desktop/dev/suter_lib/nodejs/sender_pri.key").unwrap()).unwrap()).unwrap();

    let receiver_pk = PublicKey::from_bytes(&hex::decode(fs::read("/Users/k/Desktop/dev/suter_lib/nodejs/receiver_pub.key").unwrap()).unwrap()).unwrap();

    let mut transfers = Vec::new();
    transfers.push((receiver_pk,26));
    let transaction = Transaction::<u64>::create_transaction(
        &sender_balance,
        &transfers,
        None,
        sender_pk,
        &sender_sk,
    )
    .expect("Should be able to create transaction");
    println!("交易创建成功");    


}




#[test]
fn test() {
    let mut csprng = OsRng;
    let mut csprng1 = OsRng;
    let sender_sk = SecretKey::generate_with(&mut csprng);
    fs::write("./private.key", hex::encode(sender_sk.to_bytes()));
    println!("生成私钥:{}",hex::encode(sender_sk.to_bytes()));
    let sender_pk = sender_sk.to_public();
    println!("生成公钥:{}",hex::encode(sender_pk.to_bytes()));
    let max_32 = u64::from(std::u16::MAX);
    let receiver_initial_balances: Vec<u64> = vec![1, 155, 100];
    let transaction_values: Vec<u64> = vec![8, 88, 888];
    // let receivers_info: Vec<_> = receiver_initial_balances
    //     .iter()
    //     .map(|receiver_initial_balance| {
    //         let receiver_sk = SecretKey::generate_with(&mut csprng1);
    //         let receiver_pk = receiver_sk.to_public();
    //         let receiver_initial_encrypted_balance =
    //             receiver_initial_balance.encrypt_with(receiver_pk);
    //         (
    //             receiver_sk,
    //             receiver_pk,
    //             *receiver_initial_balance,
    //             receiver_initial_encrypted_balance,
    //         )
    //     })
    //     .collect();

    let receiver_sk = SecretKey::generate_with(&mut csprng1);
    let receiver_pk = receiver_sk.to_public();
    let sender_final_balance = 10000u64;
    let transferred: u64 = transaction_values.iter().sum();
    // println!("transferred:{}",transferred);
    let sender_initial_balance: u64 = sender_final_balance + transferred;
    let sender_initial_encrypted_balance = sender_initial_balance.encrypt_with(sender_pk);
    // let transfers: Vec<(PublicKey, u64)> = receivers_info
    //     .iter()
    //     .map(|x| (x.1))
    //     .zip(transaction_values.clone())
    //     .collect();
    let transfers = vec![(receiver_pk,1)];

    println!("sender_balance: {:?}\n",&sender_initial_encrypted_balance);
    println!("transfers: {:?}\n",&transfers);
    println!("sender_pk: {:?}\n",&sender_pk);
    println!("sender_sk: {:?}======\n",&sender_sk);
    let transaction = Transaction::<u64>::create_transaction(
        &sender_initial_encrypted_balance,
        &transfers,
        None,
        sender_pk,
        &sender_sk,
    )
    .expect("Should be able to create transaction");
    println!("transaction: {:?}",transaction);
    assert!(transaction.verify_transaction().is_ok());
    // assert_eq!(
    //     transaction
    //         .try_get_sender_final_balance(&sender_sk)
    //         .unwrap(),
    //     sender_final_balance
    // );
    // let receiver_final_encrypted_balances = transaction.get_receiver_final_encrypted_balance(
    //     &receivers_info.iter().map(|x| (x.3)).collect::<Vec<_>>(),
    // );
    // for (i, sk) in receivers_info.iter().map(|x| (&x.0)).enumerate() {
    //     println!("++++++\n{:?}\n++++++准备解密：",receiver_final_encrypted_balances[i]);
    //     println!("{:?}",u64::try_decrypt_from(sk, receiver_final_encrypted_balances[i]).unwrap());
    //     assert_eq!(
    //         receivers_info[i].2 + &transaction_values[i],
    //         u64::try_decrypt_from(sk, receiver_final_encrypted_balances[i]).unwrap()
    //     )
    // }
    // for (i, sk) in receivers_info.iter().map(|x| (&x.1)).enumerate() {
    //     println!("生成公钥:{}",hex::encode(sk.to_bytes()));
    // }



}
