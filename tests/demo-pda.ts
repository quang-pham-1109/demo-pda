import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DemoPda } from "../target/types/demo_pda";

describe("demo-pda", () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DemoPda as Program<DemoPda>;

  // Sample entry data
  const restaurant = "Big U";
  const review = "The food was delicious and the service was excellent.";
  const rating = 5;

  it("Review Posted!", async () => {
    // Get the provider and the wallet. In this instance is the person who sends the review
    // anchor provides a local wallet by default for testing
    const publicKey = anchor.AnchorProvider.local().wallet.publicKey;

    // Generate the PDA for the review
    // which is the restaurant name and the public key of the person who sent the review
    // we have to also pass in the program id (which is the smart contract we are building)
    const [review_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(restaurant), publicKey.toBuffer()],
      program.programId
    );
    
    // Send the review to our local cluster
    await program.methods
      .postReview(
        restaurant, 
        review, 
        rating)
      .accounts({review: review_pda})
      .rpc();
    
    // Fetch the review data from the blockchain
    const reviewData = await program.account.review.fetch(review_pda);
    console.log('Review Data: ', reviewData);
  });
});
