var FFI= require("ffi-napi")
const fs = require('fs');
var suter = new FFI.Library('libsuter_lib.dylib', {
    'generate_keys': ['void',['string','string']],
    'encrypt_with_pubkey': ['void',['string','string','char*']],
    'decrypt_with_prikey': ['uint64',['string','string']],
    'gen_tx':['void',['string','string','string','string','string','string']],
    'verify_tx':['int8',['string']]
 });

console.log(suter.generate_keys("./sender_pub.key","./sender_pri.key"));
console.log(suter.generate_keys("./receiver_pub.key","./receiver_pri.key"));


suter.encrypt_with_pubkey("./sender_pub.key","326","ctx.txt")


const data = fs.readFileSync('./ctx.txt', 'utf8');
console.log("获取解密结果:")
console.log(suter.decrypt_with_prikey("./sender_pri.key",data))

console.log("生成隐私交易:")
suter.gen_tx(data,"./sender_pub.key","./sender_pri.key","./receiver_pub.key","26","tx.txt")
console.log(suter.verify_tx("./tx.txt"))