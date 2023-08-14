#[cfg(test)]
mod tests {
    use ark_ec::AffineRepr;
    use ark_serialize::{
        CanonicalSerialize,
        Compress
    };
    use ark_bls12_377::{
        G1Affine as G1,
        Fr
    };
    use ark_ff::PrimeField;
    use merlin::Transcript;

    #[test]
    fn test_transcript() {
        let g = G1::generator();
        let mut buffer = Vec::new();

        // 完全转换为bytes(x) || bytes(y) 低位在前， 未经压缩的时候单个坐标填充至384比特
        g.serialize_uncompressed(&mut buffer).unwrap();
        let mut g_size = g.serialized_size(Compress::No);
        assert_eq!(g_size, (384/8) * 2);

        // y坐标用0或者1来表示正负
        buffer.clear();
        g.serialize_compressed(&mut buffer).unwrap();
        g_size = g.serialized_size(Compress::Yes);
        assert_eq!(g_size, 48);

        // Fr也具有序列化方法
        buffer.clear();
        let s_280 = Fr::from(280);
        s_280.serialize_uncompressed(&mut buffer).unwrap();
        let mut vec = Vec::<u8>::from([24, 1]);
        vec.extend(vec![0; 30]);
        assert_eq!(&buffer, &vec);

        // 测试transcript生成
        let mut coin_gen1 = Transcript::new(b"Test");

        buffer.clear();
        g.serialize_compressed(&mut buffer).unwrap();
        coin_gen1.append_message(b"com1", &buffer);
        coin_gen1.append_message(b"com2", &buffer);

        let mut ch = vec![0; (Fr::MODULUS_BIT_SIZE / 8) as usize + 1];
        coin_gen1.challenge_bytes(b"com_test", ch.as_mut_slice());
        assert_eq!(ch.len(), 32);
        let ch_a = Fr::from_le_bytes_mod_order(&ch.as_slice());

        // 这个案例也能说明ark_serialize是小端序列化
        buffer.clear();
        ch_a.serialize_uncompressed(&mut buffer).unwrap();
        let ret = Fr::from_le_bytes_mod_order(&buffer.as_slice());
        assert_eq!(ret, ch_a);
    }
}