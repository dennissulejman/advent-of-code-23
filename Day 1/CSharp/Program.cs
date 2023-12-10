string zero = "zero";
string one = "one";
string two = "two";
string three = "three";
string four = "four";
string five = "five";
string six = "six";
string seven = "seven";
string eight = "eight";
string nine = "nine";

string inputFile = "../input.txt";

int partOneResult = 0;
int partTwoResult = 0;

foreach (string line in File.ReadLines(inputFile))
{
    bool firstNumberCharSet = false;
    char firstNumberChar = ' ';
    char lastNumberChar = ' ';

    foreach (char c in line)
    {
        if (char.IsDigit(c))
        {
            if (!firstNumberCharSet)
            {
                firstNumberChar = c;
                firstNumberCharSet = true;
            }
            lastNumberChar = c;
        }
    }

    partOneResult += int.Parse($"{firstNumberChar}{lastNumberChar}");
}

Func<string, char, char> StringNumberToChar = (sequence, c) =>
    sequence switch
    {
        string x when x.Contains(zero) => '0',
        string x when x.Contains(one) => '1',
        string x when x.Contains(two) => '2',
        string x when x.Contains(three) => '3',
        string x when x.Contains(four) => '4',
        string x when x.Contains(five) => '5',
        string x when x.Contains(six) => '6',
        string x when x.Contains(seven) => '7',
        string x when x.Contains(eight) => '8',
        string x when x.Contains(nine) => '9',
        _ => c
    };

foreach (string line in File.ReadLines(inputFile))
{
    char firstNumberChar = ' ';
    char lastNumberChar = ' ';
    string currentCharsSequence = "";

    foreach (char c in line)
    {
        currentCharsSequence += c;

        char currentNumberChar = StringNumberToChar(currentCharsSequence, c);

        if (char.IsDigit(currentNumberChar))
        {
            firstNumberChar = currentNumberChar;
            break;
        }
    }

    currentCharsSequence = "";

    foreach (char c in line.Reverse())
    {
        currentCharsSequence = currentCharsSequence.Insert(0, c.ToString());

        char currentNumberChar = StringNumberToChar(currentCharsSequence, c);

        if (char.IsDigit(currentNumberChar))
        {
            lastNumberChar = currentNumberChar;
            break;
        }
    }

    partTwoResult += int.Parse($"{firstNumberChar}{lastNumberChar}");
}

Console.WriteLine(partOneResult);
Console.WriteLine(partTwoResult);
