use blake3::Hasher;

pub fn hash_pair(
    left: &[u8; 32],
    right: &[u8; 32],
) -> [u8; 32] {

    let mut hasher =
        Hasher::new();

    hasher.update(left);
    hasher.update(right);

    let hash =
        hasher.finalize();

    *hash.as_bytes()
}

pub fn merkle_root(
    leaves: Vec<[u8; 32]>,
) -> [u8; 32] {

    if leaves.is_empty() {
        return [0u8; 32];
    }

    let mut current = leaves;

    while current.len() > 1 {

        let mut next = Vec::new();

        for pair in current.chunks(2) {

            let left = pair[0];

            let right =
                if pair.len() == 2 {
                    pair[1]
                } else {
                    pair[0]
                };

            next.push(
                hash_pair(&left, &right)
            );
        }

        current = next;
    }

    current[0]
}