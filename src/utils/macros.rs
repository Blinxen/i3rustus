// This macro takes a walkable buffer type (See WalkingVec)
// and a number type.
// The generated code walk for the exact size of the number type
// and then tries to convert the collected bytes to the number type.
macro_rules! walk_to_number {
    ( $array:expr, $type:ty ) => {
        <$type>::from_le_bytes(
            $array
                .walk(std::mem::size_of::<$type>())
                .try_into()
                .unwrap(),
        )
    };
}

macro_rules! walk_until_with_condition {
    ( $array:expr, $range:expr, $walking_distance:expr, $condition:expr, $walk_type:ty, $resulting_array_type:ty ) => {{
        let mut items: Vec<$resulting_array_type> = Vec::new();

        for _ in $range {
            let walked_value = walk_to_number!($array, $walk_type);
            if $condition(walked_value) {
                items.push(walked_value as $resulting_array_type);
            }
        }

        items
    }};
}

pub(crate) use walk_to_number;
pub(crate) use walk_until_with_condition;
