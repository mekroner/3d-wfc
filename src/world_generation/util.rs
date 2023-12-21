use super::*;


#[inline]
pub fn get_index(x: usize, y: usize, z: usize) -> usize {
    y * CHUNK_AREA + x * CHUNK_SIZE + z
}

#[inline]
pub fn from_index(index: usize) -> (usize, usize, usize) {
    let z = index % CHUNK_SIZE;
    let x = index / CHUNK_SIZE % CHUNK_SIZE;
    let y = index / CHUNK_AREA;

    (x,y,z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_index() {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HIGHT {
                    let index = get_index(x,y,z);
                    let (x0,y0,z0) = from_index(index);
                    assert_eq!(x, x0, "compared x={x} and x0={x0}, for index={index}");
                    assert_eq!(y, y0, "compared y={y} and y0={y0}, for index={index}");
                    assert_eq!(z, z0, "compared z={z} and z0={z0}, for index={index}");
                }
            }
        }
    }
}
