#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[allow(dead_code)]
    struct OnlyInts {
        a: u32, //4
        b: u64, //8
    }

    #[allow(dead_code)]
    struct HasBoolFlag {
        a: u32,  //4
        b: u32,  //4
        c: bool, //1
    }

    #[allow(dead_code)]
    struct HasVec {
        myvec: Vec<u8>,//24
    }

    #[test]
    //We would expect 12 in total
    //because of the alignment we end up with 16
    fn expect_12() {
        assert_ne!(12, std::mem::size_of::<OnlyInts>())
    }

    #[test]
    //We'd naively expect 4+4+1, but because of bit allignment we get 12
    fn expect_9() {
        assert_eq!(12, std::mem::size_of::<HasBoolFlag>())
    }

    #[test]
    //On a typical 64-bit architecture
    //A pointer (*const T) is 8 bytes (64 bits),
    //A usize is 8 bytes.
    fn expect_24_vec_u8() {
        let pointer = std::mem::size_of::<*mut u8>();
        let length = std::mem::size_of::<*mut u8>();
        let capacity = std::mem::size_of::<*mut u8>();

        assert_eq!(std::mem::size_of::<Vec<u8>>(), pointer + length + capacity);
    }

    #[test]
    fn both_8_bytes() {
        use std::alloc::{alloc, dealloc, Layout};
        let layout = Layout::from_size_align(8, 8).unwrap();

        // Manualy Allocate 8 bytes of memory.
        let ptr = unsafe { alloc(layout) };
        let pointer = std::mem::size_of_val(&ptr);

        unsafe {
            dealloc(ptr, layout);
        }

        let arr: [u8; 8] = [0; 8];
        let arr_size = std::mem::size_of_val(&arr);

        //This would be 32 bytes (32*8/8), since arr would default to [i32;8];
        //let arr_size = std::mem::size_of_val(&[0;8]);

        assert_eq!(pointer, arr_size);
    }

    #[test]
    //heap alloc pointer on 64-bit system
    fn heap_alloc() {
        let heap_allocated: Box<u64> = Box::new(0);

        let ptr = std::mem::size_of_val(&heap_allocated);
        let heap = std::mem::size_of::<u64>();
        assert_eq!(8, ptr);
        assert_eq!(8, heap);
    }

    #[allow(dead_code)]
    struct Mess {
        a: u32,  //4
        b: u32,  //4
        c: u64,  //8
        d: bool, //1
    }

    #[test]
    //The total size of the struct Mess is not simply the sum of its fields = 17 , we need to add allignment.
    /*
    Estimated Layout:
      a: 4 bytes (at offset 0)
      b: 4 bytes (at offset 4)
      c: 8 bytes (at offset 8)
      d: 1 byte (at offset 16)

      Padding after d: 7 bytes of padding to align the struct to an 8-byte boundary (because the largest alignment is 8 bytes).
     */
    fn mess_allignment() {
        //struct size (24 because of +7 alignment bytes)
        let size = std::mem::size_of::<Mess>();
        assert_eq!(24, size);

        //struct alignment
        let alignment = std::mem::align_of::<Mess>();
        assert_eq!(8, alignment);
    }
}
