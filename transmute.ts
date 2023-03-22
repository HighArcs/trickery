export interface Marker<Size extends number, Signed extends boolean = false> {
    value: bigint,
    size: Size,
    signed: Signed
}

export type u8 = Marker<8, false>;
export function u8(value: bigint): u8 {
  return { size: 8, signed: false, value };
}

export type i8 = Marker<8, true>;
export function i8(value: bigint): i8 {
  return { size: 8, signed: true, value };
}

export type u16 = Marker<16, false>;
export function u16(value: bigint): u16 {
  return { size: 16, signed: false, value };
}

export type i16 = Marker<16, true>;
export function i16(value: bigint): i16 {
  return { size: 16, signed: true, value };
}

export type u32 = Marker<32, false>;
export function u32(value: bigint): u32 {
  return { size: 32, signed: false, value };
}

export type i32 = Marker<32, true>;
export function i32(value: bigint): i32 {
  return { size: 32, signed: true, value };
}

export type u64 = Marker<64, false>;
export function u64(value: bigint): u64 {
  return { size: 64, signed: false, value };
}

export type i64 = Marker<64, true>;
export function i64(value: bigint): i64 {
  return { size: 64, signed: true, value };
}

export type u128 = Marker<128, false>;
export function u128(value: bigint): u128 {
  return { size: 128, signed: false, value };
}

export type i128 = Marker<128, true>;
export function i128(value: bigint): i128 {
  return { size: 128, signed: true, value };
}

export namespace Conversions {
    export type Len<T, N extends number, U extends Array<T> = []> = U['length'] extends N ? U : Len<T, N, [T, ...U]>;
    export function merge<T extends number, U extends number, S extends boolean>(buf: Array<Marker<T>>, size: U, signed: S = false as S): Marker<U, S> {
        let value = 0n;

        for (let i: number = 0; i < buf.length; i++) {
            value |= (buf[i].value << BigInt(i));
        }

        return { size, value, signed };
    }

    export function u8_u8(u8: u8): u8 {
        return u8;
    }

    export function u8_i8(u8: u8): i8 {
        return i8(u8.value);
    }

    export function u8x2_u16(u8x2: Len<u8, 2>): u16 {
        return merge(u8x2, 16);
    }


}