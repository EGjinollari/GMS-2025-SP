enum SampleData {
  Integer(i32),
  Float(f32),
  Boolean(bool),
}
struct Sample {
  name: String,
  data: SampleData
}
fn encode(sample: &Sample, buffer: &mut [u8]) {
  // Implement this!
  // Implement this!

    let name_len= sample.name.len();
    let mut tracker = 0;

    buffer[tracker..tracker + 4].copy_from_slice(&0_i32.to_le_bytes()); // Hardcoded 0 in 4 bytes

    tracker += 4;

    buffer[tracker..tracker + 4].copy_from_slice(&(name_len as i32).to_le_bytes()); // Name in 4 bytes 

    tracker += 4;

    buffer[tracker..tracker + name_len].copy_from_slice(sample.name.as_bytes()); // Name in length of name bytes

    tracker += name_len;

    let variant: i32  = match sample.data{  
        SampleData::Integer(_i) => 0,
        SampleData::Float(_f) => 1,
        SampleData::Boolean(_b) => 2,
    };

    buffer[tracker..tracker + 4].copy_from_slice(&variant.to_le_bytes()); // Varaint in 4 bytes
    
    tracker += 4;

    match sample.data{                      // Insert Data into Buffer
        SampleData::Integer(i) => {
            buffer[tracker..tracker + 4].copy_from_slice(&i.to_le_bytes());

        },
        SampleData::Float(f) => {
            buffer[tracker..tracker + 4].copy_from_slice(&f.to_le_bytes());

        },
        SampleData::Boolean(b) => {
            let b_data: u8 = match b{
                true => 1,
                false => 0,
            };

            buffer[tracker..tracker + 1].copy_from_slice(&b_data.to_le_bytes());
        },
    }
}


fn main(){}


#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_encode() {
    let sample = Sample {
      name: "accel".into(),
      data: SampleData::Integer(3),
    };
    let mut bytes = [0u8; 21];
    encode(&sample, &mut bytes);
    


    assert_eq!(bytes, [0, 0, 0, 0, 5, 0, 0, 0, 97, 99, 99, 101, 108, 0, 0, 0, 0, 3, 0, 0, 0]);
  }
}
