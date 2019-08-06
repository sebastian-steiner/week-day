# week-day
Practice your mental skills by learning to quickly calculate the day of week of an arbitrary date in the Gregorian calendar.

There are numerous tutorials online that explain the method but most of the time use different constants to remember. This version aims to be as fast as possible for ![Memoriad™](http://www.memoriad.com) problems and thus uses the **faster advanced algorithm** described at ![World Mental Calculation](https://worldmentalcalculation.com/how-to-calculate-calendar-dates/).

## Basic algorithm
To calculate the day of week we split the date into multiple parts, e.g. 01.02.2003:

1. **Day of month** e.g. **01**.02.2003
    - Just the value
2. **Month** e.g. 01.**02**.2003
    - A **month code**
3. **Year** e.g. 01.02.20**03**
    - A **year code** made up of the following calculation in which the last two digits are called *x*: *(x + x / 4) % 7*, where *%* is the **modulo** operator (remainder of division) and */* is integer division without a decimal part
4. **Century** e.g. 01.02.**20**03
    - A **century code**

You now **add day + month code + year code + century code and subtract 1 if your date is in January or February of a leap year**. Take the remainder of dividing your final number by 7 and look up the date in this table:

- 0: Sunday
- 1: Monday
- 2: Tuesday
- 3: Wednesday
- 4: Thursday
- 5: Friday
- 6: Saturday

## Differences to other methods
 In this case the constants for *months* and *centuries* is different than others such as ![this one](https://plus.maths.org/content/what-day-week-were-you-born). But since *century codes* are changed to better fit the needs of the competition, so are *year codes* (corresponding to the two least significant digits in the year). That is each *year code* is **incremented by 2** two speed up computation.
