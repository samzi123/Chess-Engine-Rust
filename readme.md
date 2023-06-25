This is a chess bot implemented with the minimax algorithm. It is built in Rust and deployed on AWS Lambda, and interacts with the Lichess API. 

To keep costs low, it was deployed "serverlessly", but this means that the bot runs at predefined intervals. At the moment, it runs every 30 minutes, in which it will accept challenges if each turn lasts at least 1 day or more, and it will make moves in all games where it is the bot's turn. This is a drawback of deploying using Lambda functions, but was used to avoid having to pay to keep a server running constantly.

Challenge it on Lichess here: https://lichess.org/@/samzi1234
Please not that it only accepts challenges where each turn lasts at least a day, and it runs every 30 mins so it may take some time to accept your challenge.