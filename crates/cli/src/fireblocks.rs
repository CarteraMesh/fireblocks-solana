#[macro_export]
macro_rules! fireblocks_sign {
    ($tx:expr, $signers:expr, $blockhash:expr) => {
        if $signers.len() == 1 {
            $tx.try_sign($signers, $blockhash)?;
        } else {
            for s in $signers {
                s.try_sign_multi_legacy(&mut $tx, $signers, $blockhash)?;
            }
        }
    };
}
