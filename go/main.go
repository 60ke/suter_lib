package main

//#cgo CFLAGS: -I${SRCDIR}/../include
//#cgo LDFLAGS: -L${SRCDIR}/../target/release/ -lsuter_lib
//#include "suter.h"
import "C"
import (
	"fmt"
	"io/ioutil"
)

func main() {
	C.generate_keys(C.CString("./sender_pub.key"), C.CString("./sender_pri.key"))
	C.generate_keys(C.CString("./receiver_pub.key"), C.CString("./receiver_pri.key"))
	C.encrypt_with_pubkey(C.CString("./sender_pub.key"), C.CString("326"), C.CString("./ctx.txt"))
	fmt.Println("解密后的值为:")
	b, err := ioutil.ReadFile("./ctx.txt") // just pass the file name
	if err != nil {
		fmt.Print(err)
	}
	fmt.Printf("%d\n", C.decrypt_with_prikey(C.CString("./sender_pri.key"), C.CString(string(b))))
	C.gen_tx(C.CString(string(b)), C.CString("./sender_pub.key"), C.CString("./sender_pri.key"), C.CString("./receiver_pub.key"), C.CString("3"), C.CString("./tx.txt"))
	if C.verify_tx(C.CString("/Users/k/Desktop/8lab/suter_lib/nodejs/tx.txt")) == 0 {
		fmt.Println("交易验证通过")
	} else {
		fmt.Println("交易验证失败")
	}

}
