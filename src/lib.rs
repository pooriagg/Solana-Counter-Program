use solana_program::{
    log::sol_log,
    pubkey::Pubkey,
    entrypoint::MAX_PERMITTED_DATA_INCREASE
};
use std::mem::size_of;

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input_buffer: *mut u8) -> u64 {
    sol_log("Solana!");

    let mut offset: usize = 0;

    // number of accounts
    let accounts_num = *(input_buffer.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    if accounts_num != 1 {
        sol_log("Only counter_account needed.");
        return 1;
    };

    let counter_account_data_len: usize;
    let counter_account_data: &mut [u8];
    let counter_account_owner: &Pubkey;
    
    // duplicate account flag
    offset += size_of::<u8>();

    // is_signer flag
    offset += size_of::<u8>();

    // is_writable flag
    offset += size_of::<u8>();

    // executable flag
    offset += size_of::<u8>();

    // 4 Bytes Padding (original data length)
    offset += size_of::<u32>();

    // counter account pubkey
    offset += size_of::<Pubkey>();

    counter_account_owner = &*(input_buffer.add(offset) as *const Pubkey);
    offset += size_of::<Pubkey>();

    // counter account lamport balance
    offset += size_of::<u64>();

    counter_account_data_len = *(input_buffer.add(offset) as *const u64) as usize; 
    offset += size_of::<u64>();

    if counter_account_data_len != 8 {
        sol_log("Counter account must have exactly size_of::<u64>()");
        return 2;
    };

    counter_account_data = std::slice::from_raw_parts_mut(
        input_buffer.add(offset),
        counter_account_data_len
    );
    offset += counter_account_data_len + MAX_PERMITTED_DATA_INCREASE;

    // Padding & Alignment
    offset += (offset as *const u8).align_offset(std::mem::align_of::<u128>());

    // Rent epoch
    offset += size_of::<u64>();

    let instruction_data_len = *(input_buffer.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    // instruction data
    offset += instruction_data_len;

    let program_id = &*(input_buffer.add(offset) as *const Pubkey);

    if counter_account_owner != program_id {
        sol_log("Counter account has invalid owner!");
        return 3;
    };

    let mut counter = u64::from_le_bytes(
        if let Ok(r) = counter_account_data.try_into() {
            r
        } else {
            return 4;
        }
    );

    counter = if let Some(c) = counter.checked_add(1) {
        c
    } else {
        sol_log("Failed to increment the counter.");
        return 5;
    };

    solana_program::program_memory::sol_memcpy(
        counter_account_data,
        counter.to_le_bytes().as_slice(),
        size_of::<u64>()
    );
    sol_log("Counter incremented.");
 
    solana_program::entrypoint::SUCCESS
}

solana_program::custom_panic_default!();
solana_program::custom_heap_default!();
