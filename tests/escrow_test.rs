#![allow(unexpected_cfgs)]
#![allow(unused)]
use anchor_litesvm::{AnchorLiteSVM, TestHelpers};
use litesvm_utils::AssertionHelpers;
// use litesvm_utils::{AssertionHelpers, TestHelpers};
use solana_sdk::signature::Signer;
use anchor_lang::system_program;
use spl_associated_token_account::get_associated_token_address;
use spl_token;
use escrow_1::ID;
// anchor_lang::declare_program!(escrow_1);

#[test]

pub fn test_make_and_take(){
    let pk= escrow_1::ID ;
    let program_id=anchor_litesvm::Pubkey::new_from_array(pk.to_bytes())
    ;

    let mut ctx = AnchorLiteSVM::build_with_program(
        program_id ,
        include_bytes!("../target/deploy/escrow_1.so"),

    );

    let mut maker = ctx.svm.
    create_funded_account(10_000_000_000).unwrap();
    
    let mut taker = ctx.svm.
    create_funded_account(10_000_000_000).unwrap();

    let mint_a=ctx.svm.create_token_mint(&maker, 9).unwrap();
    let mint_b=ctx.svm.create_token_mint(&maker, 9).unwrap();


    let maker_ata_a=ctx.svm.
    create_associated_token_account(&mint_a.pubkey(), &maker).unwrap();

    ctx.svm.
    mint_to(&mint_a.pubkey(), &maker_ata_a,&maker,1_000_000_000).unwrap();

    
    let taker_ata_b=ctx.svm.
    create_associated_token_account(&mint_b.pubkey(), &taker).unwrap();

    match ctx.svm.
    mint_to(&mint_b.pubkey(), &taker_ata_b,&maker,500_000_000){
        (_)=>{

        },
        (Err(e))=>{
            println!("{:?}",e);
        }
    };

    let seed: u64 = 42;
    let escrow_pda = ctx.svm.get_pda(
        &[b"escrow", maker.pubkey().as_ref(), &seed.to_le_bytes()],
        &program_id,
    );

    let vault = get_associated_token_address(&escrow_pda, &mint_a.pubkey());

    let receive= 500_000_000;  // 0.5 tokens
    let amount= 1_000_000_000;

    let make_i=ctx.program().accounts(escrow_1::accounts::Maker{
        maker:maker.pubkey(),
        escrow_acc:escrow_pda,
        token_a:mint_a.pubkey(),
        token_b:mint_b.pubkey(),
        maker_ata_a:maker_ata_a,
        vault:vault,
        associated_token_program:spl_associated_token_account::id(),
        token_program:spl_token::id(),
        system_program:system_program::ID

    }).args(escrow_1::instruction::Make{
        seed,
        amount,
        receive
    }).instruction().unwrap();

    ctx.execute_instruction(make_i, &[&maker])
    .unwrap().assert_success();

    println!("sdada");
    // Verify escrow was created and tokens were transferred
    assert!(ctx.account_exists(&escrow_pda), "Escrow account should exist");    
    ctx.svm.assert_token_balance(&vault, 1_000_000_000);
    ctx.svm.assert_token_balance(&maker_ata_a, 0);  

    println!("sdada");

    let taker_ata_a = get_associated_token_address(&taker.pubkey(), &mint_a.pubkey());
    let maker_ata_b = get_associated_token_address(&maker.pubkey(), &mint_b.pubkey());

    ctx.svm.assert_token_balance(&vault, 1_000_000_000);
ctx.svm.assert_token_balance(&taker_ata_b, 500_000_000);

    let take_ix = ctx.program()
    .accounts(escrow_1::accounts::Taker {
        taker: taker.pubkey(),
        maker: maker.pubkey(),
        escrow_acc: escrow_pda,
        token_a: mint_a.pubkey(),
        token_b: mint_b.pubkey(),
        vault:vault,
        taker_ata_a:taker_ata_a,
        taker_ata_b:taker_ata_b,
        maker_ata_b:maker_ata_b,
        associated_token_program: spl_associated_token_account::id(),
        token_program: spl_token::id(),
        system_program: system_program::ID,
    })
    .args(escrow_1::instruction::Take{})
    .instruction()
    .unwrap();

    println!("sdada");
    ctx.execute_instruction(take_ix, &[&taker])
    .unwrap()
    .assert_success();
println!("-------------------");

// ============================================================================
// 6. Verify final state
// ============================================================================

// Verify accounts were closed
ctx.svm.assert_account_closed(&escrow_pda);
ctx.svm.assert_account_closed(&vault);

// Verify token balances after the swap
ctx.svm.assert_token_balance(&taker_ata_a, 1_000_000_000); // Taker received mint_a tokens
ctx.svm.assert_token_balance(&taker_ata_b, 0);             // Taker sent all mint_b tokens
ctx.svm.assert_token_balance(&maker_ata_b, 500_000_000);
}