pub fn decode_cobs(data: &Vec<u8>) -> (Vec<u8>,Vec<u8>) {
    let mut out = Vec::new();
    let mut enc_idx = 0;
    let mut next_zero = 0;
    let mut next_is_overhead = true;
    let mut is_end = false;
    while enc_idx < data.len() {
        if next_zero != 0{
            out.push(data[enc_idx]);
            enc_idx += 1;
        }else{
            if data[enc_idx] == 0{
                is_end = true;
                break;
            }
            if !next_is_overhead{
                out.push(0);
            }
            next_zero = data[enc_idx] as usize;
            enc_idx += 1;

            if next_zero == 0xff{
                next_is_overhead = true;
            }else{
                next_is_overhead = false;
            }
        }
        next_zero -= 1;
    }
    if !is_end{
        return (Vec::new(),data.to_vec());
    }
    (out,data[enc_idx+1..].to_vec())
}

#[allow(dead_code)]
pub fn encode_cobs(data:&Vec<u8>)->Vec<u8>{
    let mut enc_data=vec![0];
    let mut enc_idx=0;
    let mut code_idx=0;
    for d in data{
        if *d==0{
            enc_data[enc_idx]=code_idx+1;
            enc_data.push(0);
            enc_idx+=1;
            code_idx=0;
        }else{
            enc_data.push(*d);
            if code_idx==0xff{
                enc_data[enc_idx]=code_idx;
                enc_data.push(0);
                enc_idx+=1;
            }
        }
    }
    enc_data[enc_idx]=code_idx+1;
    enc_data.push(0);
    enc_data
}