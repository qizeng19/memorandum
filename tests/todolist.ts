import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Todolist } from "../target/types/todolist";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("todolist", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Todolist as Program<Todolist>;

    let user = anchor.web3.Keypair.generate();
    let userState;
    let listItem;
    let availableIndex;

    before(async () => {
        const signature = await program.provider.connection.requestAirdrop(
            user.publicKey,
            5 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL = 2 billion lamports
        );
        await program.provider.connection.confirmTransaction(signature);
        userState = PublicKey.findProgramAddressSync(
            [Buffer.from("user"), user.publicKey.toBuffer()],
            program.programId
        )[0];
    });
    it("initialize", async () => {
        // Add your test here.
        const tx = await program.methods
            .initialize(20)
            .accounts({
                user: user.publicKey,
                userState: userState,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        const userStateData = await program.account.userState.fetch(userState);

        assert.equal(userStateData.user.toBase58(), user.publicKey.toBase58());
        assert.equal(userStateData.indexArray.length, 20);
    });

    it("first time : find available index and add", async () => {
        const availableIndex = await program.methods
            .findAvailableIndex()
            .accounts({
                user: user.publicKey,
                userState: userState,
            })
            .view();
        listItem = PublicKey.findProgramAddressSync(
            [
                Buffer.from("list_item"),
                user.publicKey.toBuffer(),
                new BN(availableIndex).toArrayLike(Buffer, "le", 1), // 这里代表1个字节 与程序类型u8对应
            ],
            program.programId
        )[0];
        const listItemInfo = await program.provider.connection.getAccountInfo(listItem);
        console.log("listItemInfo1", listItemInfo); // 这里打印的是null
        const tx = await program.methods
            .add("test", new BN(availableIndex))
            .accounts({
                user: user.publicKey,
                userState: userState,
                listItem: listItem,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        const latestBlockHash = await program.provider.connection.getLatestBlockhash();
        await program.provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: tx,
        });
        const listItemInfo2 = await program.provider.connection.getAccountInfo(listItem);
        console.log("listItemInfo2", listItemInfo2);
        const listItemData = await program.account.listItem.fetch(listItem);
        console.log("listItemData", listItemData);
        assert.equal(listItemData.content, "test");
    });

    it("update", async () => {
        const availableIndex = 0;

        const tx = await program.methods
            .update(new BN(availableIndex), "test23", true)
            .accounts({
                user: user.publicKey,
                listItem: listItem,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();

        const listItemData = await program.account.listItem.fetch(listItem);
        console.log("listItemData", listItemData);
        assert.equal(listItemData.content, "test23");
        assert.equal(listItemData.isCompleted, true);
    });
    it("second time : find available index and add", async () => {
        const availableIndex2 = await program.methods
            .findAvailableIndex()
            .accounts({
                user: user.publicKey,
                userState: userState,
            })
            .view();
        console.log("availableIndex2", availableIndex2);
        const listItem2 = PublicKey.findProgramAddressSync(
            [
                Buffer.from("list_item"),
                user.publicKey.toBuffer(),
                new BN(availableIndex2).toArrayLike(Buffer, "le", 1), // 这里代表1个字节 与程序类型u8对应
            ],
            program.programId
        )[0];
        await program.methods
            .add("this is a test", new BN(availableIndex2))
            .accounts({
                user: user.publicKey,
                userState: userState,
                listItem: listItem2,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();
        const listItemState2 = await program.account.listItem.fetch(listItem2);
        const listItemState = await program.account.listItem.fetch(listItem);
        const indexArray = await program.account.userState.fetch(userState);
        console.log("indexArray", indexArray);
        console.log("listItemState2", listItemState2);
        console.log("listItemState", listItemState);
    });
    it("get_list1", async () => {
        const list = await program.methods
            .getList()
            .accounts({
                user: user.publicKey,
                userState: userState,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .view();
        console.log("list", list);
    });

    it("delete the first item", async () => {
        const availableIndex = 0;
        const tx = await program.methods
            .delete(new BN(availableIndex))
            .accounts({
                user: user.publicKey,
                userState: userState,
                listItem: listItem,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([user])
            .rpc();
        const listItemInfo = await program.provider.connection.getAccountInfo(listItem);
        const userStateData = await program.account.userState.fetch(userState);
        console.log("listItemInfo", listItemInfo);
        console.log("userStateData", userStateData);
    });
    it("get_list2", async () => {
        const list = await program.methods
            .getList()
            .accounts({
                user: user.publicKey,
                userState: userState,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .view();
        console.log("list", list);
    });
});
