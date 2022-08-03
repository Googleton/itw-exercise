import BountyHunter from "@/models/bounty_hunter";

export default interface EmpireData {
  countdown: number;
  bounty_hunters: BountyHunter[];
}
