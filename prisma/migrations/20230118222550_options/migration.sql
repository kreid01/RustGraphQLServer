/*
  Warnings:

  - Made the column `user_id` on table `Deck` required. This step will fail if there are existing NULL values in that column.

*/
-- DropForeignKey
ALTER TABLE "Deck" DROP CONSTRAINT "Deck_user_id_fkey";

-- AlterTable
ALTER TABLE "Deck" ALTER COLUMN "user_id" SET NOT NULL;

-- AddForeignKey
ALTER TABLE "Deck" ADD CONSTRAINT "Deck_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
