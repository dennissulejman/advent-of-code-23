string inputFile = "../input.txt";

const string blue = "blue";
const string green = "green";
const string red = "red";

int maxBlueCubes = 14;
int maxGreenCubes = 13;
int maxRedCubes = 12;

int partOneResult = 0;
int partTwoResult = 0;

static void SetCubeAmounts(ref int cubeAmount, int currentLineAmount)
{
    if (cubeAmount < currentLineAmount)
    {
        cubeAmount = currentLineAmount;
    }
}

foreach (string line in File.ReadLines(inputFile))
{
    string[]? gameAndRounds = line.Split(':', ';');
    int gameId = int.Parse(gameAndRounds[0].Remove(0, 5));
    bool gamePossible = true;

    int leastAmountOfBlueCubesNeeded = 0;
    int leastAmountOfGreenCubesNeeded = 0;
    int leastAmountOfRedCubesNeeded = 0;

    foreach (string round in gameAndRounds.Skip(1))
    {
        string[]? cubeAmounts = round.Split(',', StringSplitOptions.TrimEntries);
        int highestAmountOfBlueCubes = 0;
        int highestAmountOfGreenCubes = 0;
        int highestAmountOfRedCubes = 0;

        foreach (string cubeAmount in cubeAmounts)
        {
            string[] amountColorSplit = cubeAmount.Split(" ");
            int amount = int.Parse(amountColorSplit[0]);
            string color = amountColorSplit[1];

            switch (color)
            {
                case blue:
                    SetCubeAmounts(ref highestAmountOfBlueCubes, amount);
                    SetCubeAmounts(ref leastAmountOfBlueCubesNeeded, amount);
                    break;
                case green:
                    SetCubeAmounts(ref highestAmountOfGreenCubes, amount);
                    SetCubeAmounts(ref leastAmountOfGreenCubesNeeded, amount);
                    break;
                case red:
                    SetCubeAmounts(ref highestAmountOfRedCubes, amount);
                    SetCubeAmounts(ref leastAmountOfRedCubesNeeded, amount);
                    break;
                default:
                    Console.WriteLine($"No operation available for {color}");
                    break;
            }
        }

        if (gamePossible)
        {
            gamePossible =
                highestAmountOfBlueCubes <= maxBlueCubes
                && highestAmountOfGreenCubes <= maxGreenCubes
                && highestAmountOfRedCubes <= maxRedCubes;
        }
    }

    if (gamePossible)
    {
        partOneResult += gameId;
    }

    partTwoResult +=
        leastAmountOfBlueCubesNeeded * leastAmountOfGreenCubesNeeded * leastAmountOfRedCubesNeeded;
}

Console.WriteLine(partOneResult);
Console.WriteLine(partTwoResult);
