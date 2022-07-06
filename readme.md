基于suter_proof的lib库

## 编译
**需安装rust**

`make`


## 函数及对应输入参数说明:

### `void generate_keys(char* pub,char* pri)`

生成公私钥,无返回参数。

输入参数说明:
```
pub为待保存的公钥名，string类型
pri为待保存的私钥名，string类型
```
### `void encrypt_with_pubkey(char* pub,char* value,char* out)`

根据公钥进行加密,无返回参数。

输入参数说明:
```
pub为公钥文件名，string类型
value为待加密的数值，string类型
out为待保存的密文文件名，string类型
```

### `void gen_tx(char* balance,char* from_pub,char* from_pri,char* to_pub,char* to_value,char* tx_name)`

生成转账隐私交易(已包含交易证明)，无返回类型

输入参数说明:

```
balance为from账户余额，string类型
from_pub为发送者的公钥文件名，string类型
from_pri为发送者的私钥文件名，string类型
to_pub为接收者者的公钥文件名，string类型
to_value为明文转账金额，但类型为string类型
tx_name为待保存的隐私交易文件名，string类型
```


### `int8 verify_tx(char* tx_name)`

隐私交易验证(验证交易的proof)，返回int8类型，0代表验证通过，其它数值则表明验证失败

输入参数说明:

```
tx_name待验证交易文件名，string类型
```

## Example
### Nodejs
nodejs目录下的，suter_proof.js为该库的js调用示例
