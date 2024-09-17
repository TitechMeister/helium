def cobs_decode(enc_data:list[int]):
    dec_data:list[int] = []
    enc_idx = 0

    next_0x00 = 0
    next_is_overhead = True
    is_end = False

    while enc_idx < len(enc_data):
        if next_0x00 != 0:
            dec_data.append(enc_data[enc_idx])
            enc_idx += 1
        else:
            if enc_data[enc_idx] == 0x00:
                is_end = True
                # 終端コード(0x00)発見時は処理を終了する。
                break

            if next_is_overhead == True:
                pass
            else:
                dec_data.append(0)

            next_0x00 = enc_data[enc_idx]
            enc_idx += 1

            if next_0x00 == 0xff:
                next_is_overhead = True
            else:
                next_is_overhead = False
        next_0x00 -= 1

    if is_end == False:
        # 終端コード(0x00)が見つからなかった場合は、
        # []を返す。
        return [], enc_data
    return dec_data, enc_data[enc_idx+1:]


def cobs_encode(data:list[int]):
    enc_data = []
    enc_data.append(0)
    enc_idx = 0
    code_idx = 0
    for d in data:
        if d == 0:
            enc_data[enc_idx] = code_idx + 1
            enc_data.append(0)
            enc_idx = len(enc_data) - 1
            code_idx = 0
        else:
            enc_data.append(d)
            code_idx += 1
            if code_idx == 0xff:
                enc_data[enc_idx] = code_idx
                enc_data.append(0)
                enc_idx = len(enc_data) - 1
                code_idx = 0
    enc_data[enc_idx] = code_idx + 1
    enc_data.append(0)
    return enc_data