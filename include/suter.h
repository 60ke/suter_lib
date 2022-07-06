void generate_keys(char* pub,char* pri);
int verify_tx(char* tx_name);
void encrypt_with_pubkey(char* pub,char* value,char* out);
unsigned int decrypt_with_prikey(char* pri,char* value);
void gen_tx(char* balance,char* from_pub,char* from_pri,char* to_pub,char* to_value,char* tx_name);
int verify_tx(char* tx_name);