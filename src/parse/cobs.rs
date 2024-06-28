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
    let mut enc_data=vec![];
    let mut buf=[0u8;256];
    let mut buf_idx=0;
    for d in data{
        if *d==0{
            enc_data.push(buf_idx as u8+1);
            for b in &buf[0..buf_idx]{
                enc_data.push(*b);
            }
            buf_idx=0;
        }else{
            buf[buf_idx]=*d;
            buf_idx+=1;
        }
    }
    enc_data.push(buf_idx as u8+1);
    for b in &buf[0..buf_idx]{
        enc_data.push(*b);
    }
    enc_data.push(0);
    enc_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cobs(){
        let data=vec![
            0x00,0x00,0x01,0x02,0x33
        ];
        let enc_data=encode_cobs(&data);
        println!();
        let (dec_data,rest)=decode_cobs(&enc_data);
        assert_eq!(data,dec_data);
        assert_eq!(rest.len(),0);
    }
}