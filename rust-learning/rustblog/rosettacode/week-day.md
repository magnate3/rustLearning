
# Day of the week

[![Task](https://rosettacode.org/mw/images/thumb/b/ba/Rcode-button-task-crushed.png/64px-Rcode-button-task-crushed.png)](https://rosettacode.org/wiki/Category:Solutions_by_Programming_Task "Category:Solutions by Programming Task")

**Day of the week**  
You are encouraged to  [solve this task](https://rosettacode.org/wiki/Rosetta_Code:Solve_a_Task "Rosetta Code:Solve a Task")  according to the task description, using any language you may know.

A company decides that whenever Xmas falls on a Sunday they will give their workers all extra paid holidays so that, together with any public holidays, workers will not have to work the following week (between the 25th of December and the first of January).

  

Task

**In what years between 2008 and 2121 will the 25th of December be a Sunday?**

Using any standard date handling libraries of your programming language; compare the dates calculated with the output of other languages to discover any anomalies in the handling of dates which may be due to, for example, overflow in types used to represent dates/times similar to [y2k](https://en.wikipedia.org/wiki/Y2k#See_also "wp:Y2k")  type problems.  
  

## Contents

[[hide](https://rosettacode.org/wiki/Day_of_the_week#)]

-   [1  11l](https://rosettacode.org/wiki/Day_of_the_week#11l)
-   [2  360 Assembly](https://rosettacode.org/wiki/Day_of_the_week#360_Assembly)
-   [3  ABAP](https://rosettacode.org/wiki/Day_of_the_week#ABAP)
-   [4  Action!](https://rosettacode.org/wiki/Day_of_the_week#Action.21)
-   [5  Ada](https://rosettacode.org/wiki/Day_of_the_week#Ada)
-   [6  ALGOL 68](https://rosettacode.org/wiki/Day_of_the_week#ALGOL_68)
-   [7  ALGOL-M](https://rosettacode.org/wiki/Day_of_the_week#ALGOL-M)
-   [8  AppleScript](https://rosettacode.org/wiki/Day_of_the_week#AppleScript)
-   [9  Arc](https://rosettacode.org/wiki/Day_of_the_week#Arc)
-   [10  Arturo](https://rosettacode.org/wiki/Day_of_the_week#Arturo)
-   [11  AutoHotkey](https://rosettacode.org/wiki/Day_of_the_week#AutoHotkey)
-   [12  AutoIt](https://rosettacode.org/wiki/Day_of_the_week#AutoIt)
-   [13  AWK](https://rosettacode.org/wiki/Day_of_the_week#AWK)
-   [14  BASIC](https://rosettacode.org/wiki/Day_of_the_week#BASIC)
    -   [14.1  BaCon](https://rosettacode.org/wiki/Day_of_the_week#BaCon)
    -   [14.2  FreeBASIC](https://rosettacode.org/wiki/Day_of_the_week#FreeBASIC)
    -   [14.3  IS-BASIC](https://rosettacode.org/wiki/Day_of_the_week#IS-BASIC)
    -   [14.4  Sinclair ZX81 BASIC](https://rosettacode.org/wiki/Day_of_the_week#Sinclair_ZX81_BASIC)
-   [15  Batch File](https://rosettacode.org/wiki/Day_of_the_week#Batch_File)
-   [16  BBC BASIC](https://rosettacode.org/wiki/Day_of_the_week#BBC_BASIC)
-   [17  bc](https://rosettacode.org/wiki/Day_of_the_week#bc)
-   [18  Befunge](https://rosettacode.org/wiki/Day_of_the_week#Befunge)
-   [19  Bracmat](https://rosettacode.org/wiki/Day_of_the_week#Bracmat)
-   [20  C](https://rosettacode.org/wiki/Day_of_the_week#C)
-   [21  C++](https://rosettacode.org/wiki/Day_of_the_week#C.2B.2B)
-   [22  C#](https://rosettacode.org/wiki/Day_of_the_week#C.23)
-   [23  Clojure](https://rosettacode.org/wiki/Day_of_the_week#Clojure)
-   [24  COBOL](https://rosettacode.org/wiki/Day_of_the_week#COBOL)
-   [25  CoffeeScript](https://rosettacode.org/wiki/Day_of_the_week#CoffeeScript)
-   [26  ColdFusion](https://rosettacode.org/wiki/Day_of_the_week#ColdFusion)
-   [27  Common Lisp](https://rosettacode.org/wiki/Day_of_the_week#Common_Lisp)
-   [28  Component Pascal](https://rosettacode.org/wiki/Day_of_the_week#Component_Pascal)
-   [29  D](https://rosettacode.org/wiki/Day_of_the_week#D)
-   [30  Delphi](https://rosettacode.org/wiki/Day_of_the_week#Delphi)
-   [31  ECL](https://rosettacode.org/wiki/Day_of_the_week#ECL)
-   [32  Elixir](https://rosettacode.org/wiki/Day_of_the_week#Elixir)
-   [33  Erlang](https://rosettacode.org/wiki/Day_of_the_week#Erlang)
-   [34  ERRE](https://rosettacode.org/wiki/Day_of_the_week#ERRE)
-   [35  Euphoria](https://rosettacode.org/wiki/Day_of_the_week#Euphoria)
-   [36  F#](https://rosettacode.org/wiki/Day_of_the_week#F.23)
-   [37  Factor](https://rosettacode.org/wiki/Day_of_the_week#Factor)
-   [38  FBSL](https://rosettacode.org/wiki/Day_of_the_week#FBSL)
-   [39  Fōrmulæ](https://rosettacode.org/wiki/Day_of_the_week#F.C5.8Drmul.C3.A6)
-   [40  Forth](https://rosettacode.org/wiki/Day_of_the_week#Forth)
-   [41  Fortran](https://rosettacode.org/wiki/Day_of_the_week#Fortran)
-   [42  Gambas](https://rosettacode.org/wiki/Day_of_the_week#Gambas)
-   [43  GAP](https://rosettacode.org/wiki/Day_of_the_week#GAP)
-   [44  Go](https://rosettacode.org/wiki/Day_of_the_week#Go)
-   [45  Groovy](https://rosettacode.org/wiki/Day_of_the_week#Groovy)
-   [46  Haskell](https://rosettacode.org/wiki/Day_of_the_week#Haskell)
-   [47  HicEst](https://rosettacode.org/wiki/Day_of_the_week#HicEst)
-   [48  Icon and Unicon](https://rosettacode.org/wiki/Day_of_the_week#Icon_and_Unicon)
-   [49  J](https://rosettacode.org/wiki/Day_of_the_week#J)
-   [50  Java](https://rosettacode.org/wiki/Day_of_the_week#Java)
-   [51  JavaScript](https://rosettacode.org/wiki/Day_of_the_week#JavaScript)
    -   [51.1  ES5](https://rosettacode.org/wiki/Day_of_the_week#ES5)
        -   [51.1.1  Iteration](https://rosettacode.org/wiki/Day_of_the_week#Iteration)
        -   [51.1.2  Functional composition](https://rosettacode.org/wiki/Day_of_the_week#Functional_composition)
    -   [51.2  ES6](https://rosettacode.org/wiki/Day_of_the_week#ES6)
-   [52  jq](https://rosettacode.org/wiki/Day_of_the_week#jq)
-   [53  Jsish](https://rosettacode.org/wiki/Day_of_the_week#Jsish)
-   [54  Julia](https://rosettacode.org/wiki/Day_of_the_week#Julia)
-   [55  K](https://rosettacode.org/wiki/Day_of_the_week#K)
-   [56  Kotlin](https://rosettacode.org/wiki/Day_of_the_week#Kotlin)
-   [57  Lasso](https://rosettacode.org/wiki/Day_of_the_week#Lasso)
-   [58  Liberty BASIC](https://rosettacode.org/wiki/Day_of_the_week#Liberty_BASIC)
-   [59  Lingo](https://rosettacode.org/wiki/Day_of_the_week#Lingo)
-   [60  LiveCode](https://rosettacode.org/wiki/Day_of_the_week#LiveCode)
-   [61  Logo](https://rosettacode.org/wiki/Day_of_the_week#Logo)
-   [62  Lua](https://rosettacode.org/wiki/Day_of_the_week#Lua)
    -   [62.1  Without external modules](https://rosettacode.org/wiki/Day_of_the_week#Without_external_modules)
-   [63  M2000 Interpreter](https://rosettacode.org/wiki/Day_of_the_week#M2000_Interpreter)
-   [64  M4](https://rosettacode.org/wiki/Day_of_the_week#M4)
-   [65  Maple](https://rosettacode.org/wiki/Day_of_the_week#Maple)
-   [66  Mathematica / Wolfram Language](https://rosettacode.org/wiki/Day_of_the_week#Mathematica_.2F_Wolfram_Language)
-   [67  MATLAB / Octave](https://rosettacode.org/wiki/Day_of_the_week#MATLAB_.2F_Octave)
-   [68  Maxima](https://rosettacode.org/wiki/Day_of_the_week#Maxima)
-   [69  Modula-3](https://rosettacode.org/wiki/Day_of_the_week#Modula-3)
-   [70  МК-61/52](https://rosettacode.org/wiki/Day_of_the_week#.D0.9C.D0.9A-61.2F52)
-   [71  MUMPS](https://rosettacode.org/wiki/Day_of_the_week#MUMPS)
-   [72  NetRexx](https://rosettacode.org/wiki/Day_of_the_week#NetRexx)
    -   [72.1  Comparison of Some Common Day-of-Week Algorithms](https://rosettacode.org/wiki/Day_of_the_week#Comparison_of_Some_Common_Day-of-Week_Algorithms)
-   [73  Nim](https://rosettacode.org/wiki/Day_of_the_week#Nim)
-   [74  Oberon-2](https://rosettacode.org/wiki/Day_of_the_week#Oberon-2)
-   [75  Objective-C](https://rosettacode.org/wiki/Day_of_the_week#Objective-C)
-   [76  OCaml](https://rosettacode.org/wiki/Day_of_the_week#OCaml)
    -   [76.1  With a dedicated library](https://rosettacode.org/wiki/Day_of_the_week#With_a_dedicated_library)
-   [77  Oforth](https://rosettacode.org/wiki/Day_of_the_week#Oforth)
-   [78  ooRexx](https://rosettacode.org/wiki/Day_of_the_week#ooRexx)
-   [79  PARI/GP](https://rosettacode.org/wiki/Day_of_the_week#PARI.2FGP)
-   [80  Pascal](https://rosettacode.org/wiki/Day_of_the_week#Pascal)
-   [81  Peloton](https://rosettacode.org/wiki/Day_of_the_week#Peloton)
-   [82  Perl](https://rosettacode.org/wiki/Day_of_the_week#Perl)
-   [83  Perl 6](https://rosettacode.org/wiki/Day_of_the_week#Perl_6)
-   [84  Phix](https://rosettacode.org/wiki/Day_of_the_week#Phix)
-   [85  PHP](https://rosettacode.org/wiki/Day_of_the_week#PHP)
-   [86  PicoLisp](https://rosettacode.org/wiki/Day_of_the_week#PicoLisp)
-   [87  Pike](https://rosettacode.org/wiki/Day_of_the_week#Pike)
-   [88  PL/I](https://rosettacode.org/wiki/Day_of_the_week#PL.2FI)
-   [89  PowerShell](https://rosettacode.org/wiki/Day_of_the_week#PowerShell)
    -   [89.1  Find Christmas holiday for any day and/or year](https://rosettacode.org/wiki/Day_of_the_week#Find_Christmas_holiday_for_any_day_and.2For_year)
-   [90  Prolog](https://rosettacode.org/wiki/Day_of_the_week#Prolog)
-   [91  PureBasic](https://rosettacode.org/wiki/Day_of_the_week#PureBasic)
-   [92  Python](https://rosettacode.org/wiki/Day_of_the_week#Python)
-   [93  R](https://rosettacode.org/wiki/Day_of_the_week#R)
-   [94  Racket](https://rosettacode.org/wiki/Day_of_the_week#Racket)
-   [95  REBOL](https://rosettacode.org/wiki/Day_of_the_week#REBOL)
-   [96  Red](https://rosettacode.org/wiki/Day_of_the_week#Red)
-   [97  REXX](https://rosettacode.org/wiki/Day_of_the_week#REXX)
    -   [97.1  using DATE weekday](https://rosettacode.org/wiki/Day_of_the_week#using_DATE_weekday)
    -   [97.2  using DATE base](https://rosettacode.org/wiki/Day_of_the_week#using_DATE_base)
    -   [97.3  using DATE iso](https://rosettacode.org/wiki/Day_of_the_week#using_DATE_iso)
    -   [97.4  old school DOW](https://rosettacode.org/wiki/Day_of_the_week#old_school_DOW)
-   [98  Ring](https://rosettacode.org/wiki/Day_of_the_week#Ring)
-   [99  Ruby](https://rosettacode.org/wiki/Day_of_the_week#Ruby)
-   [100  Run BASIC](https://rosettacode.org/wiki/Day_of_the_week#Run_BASIC)
-   [101  Rust](https://rosettacode.org/wiki/Day_of_the_week#Rust)
-   [102  SAS](https://rosettacode.org/wiki/Day_of_the_week#SAS)
-   [103  S-BASIC](https://rosettacode.org/wiki/Day_of_the_week#S-BASIC)
-   [104  Scala](https://rosettacode.org/wiki/Day_of_the_week#Scala)
    -   [104.1  JDK (discouraged)](https://rosettacode.org/wiki/Day_of_the_week#JDK_.28discouraged.29)
    -   [104.2  JDK >= 8 (recommended)](https://rosettacode.org/wiki/Day_of_the_week#JDK_.3E.3D_8_.28recommended.29)
        -   [104.2.1  Naive programming](https://rosettacode.org/wiki/Day_of_the_week#Naive_programming)
        -   [104.2.2  Idiomatic programming](https://rosettacode.org/wiki/Day_of_the_week#Idiomatic_programming)
        -   [104.2.3  Tail recursion](https://rosettacode.org/wiki/Day_of_the_week#Tail_recursion)
-   [105  Scheme](https://rosettacode.org/wiki/Day_of_the_week#Scheme)
-   [106  Seed7](https://rosettacode.org/wiki/Day_of_the_week#Seed7)
-   [107  Sidef](https://rosettacode.org/wiki/Day_of_the_week#Sidef)
-   [108  Smalltalk](https://rosettacode.org/wiki/Day_of_the_week#Smalltalk)
-   [109  SQL](https://rosettacode.org/wiki/Day_of_the_week#SQL)
-   [110  Stata](https://rosettacode.org/wiki/Day_of_the_week#Stata)
    -   [110.1  Mata](https://rosettacode.org/wiki/Day_of_the_week#Mata)
-   [111  Suneido](https://rosettacode.org/wiki/Day_of_the_week#Suneido)
-   [112  Standard ML](https://rosettacode.org/wiki/Day_of_the_week#Standard_ML)
-   [113  Swift](https://rosettacode.org/wiki/Day_of_the_week#Swift)
-   [114  Tcl](https://rosettacode.org/wiki/Day_of_the_week#Tcl)
-   [115  TI-83 BASIC](https://rosettacode.org/wiki/Day_of_the_week#TI-83_BASIC)
-   [116  TUSCRIPT](https://rosettacode.org/wiki/Day_of_the_week#TUSCRIPT)
-   [117  UNIX Shell](https://rosettacode.org/wiki/Day_of_the_week#UNIX_Shell)
    -   [117.1  With GNU date](https://rosettacode.org/wiki/Day_of_the_week#With_GNU_date)
    -   [117.2  With GNU date and GNU seq (UnixPipes)](https://rosettacode.org/wiki/Day_of_the_week#With_GNU_date_and_GNU_seq_.28UnixPipes.29)
    -   [117.3  With Unix cal](https://rosettacode.org/wiki/Day_of_the_week#With_Unix_cal)
    -   [117.4  With zsh](https://rosettacode.org/wiki/Day_of_the_week#With_zsh)
-   [118  Ursala](https://rosettacode.org/wiki/Day_of_the_week#Ursala)
-   [119  VBA](https://rosettacode.org/wiki/Day_of_the_week#VBA)
-   [120  VBScript](https://rosettacode.org/wiki/Day_of_the_week#VBScript)
-   [121  Vedit macro language](https://rosettacode.org/wiki/Day_of_the_week#Vedit_macro_language)
-   [122  Visual Objects](https://rosettacode.org/wiki/Day_of_the_week#Visual_Objects)
-   [123  Wortel](https://rosettacode.org/wiki/Day_of_the_week#Wortel)
-   [124  XPL0](https://rosettacode.org/wiki/Day_of_the_week#XPL0)
-   [125  Yabasic](https://rosettacode.org/wiki/Day_of_the_week#Yabasic)
-   [126  zkl](https://rosettacode.org/wiki/Day_of_the_week#zkl)
-   [127  zonnon](https://rosettacode.org/wiki/Day_of_the_week#zonnon)
-   [128  ZX Spectrum Basic](https://rosettacode.org/wiki/Day_of_the_week#ZX_Spectrum_Basic)

## [11l](https://rosettacode.org/wiki/Category:11l "Category:11l")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=1 "Edit section: 11l")]

print((2008..2121).filter(y -> Time(y, 12, 25).strftime(‘%w’) == ‘0’))

Output:

[2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

## [360 Assembly](https://rosettacode.org/wiki/Category:360_Assembly "Category:360 Assembly")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=2 "Edit section: 360 Assembly")]

**Translation of**:  [REXX](https://rosettacode.org/wiki/Day_of_the_week#REXX)

The program uses two ASSIST macro (XDECO,XPRNT) to keep the code as short as possible.

*        Day of the week           06/07/2016  
DOW      CSECT  
         USING  DOW,R15            base register  
         LA     R6,2008            year=2008  
LOOP     C      R6,=F'2121'        do year=2008 to 2121  
         BH     ELOOP              .  
         LR     R7,R6              y=year  
         LA     R8,12              m=12  
         LA     R9,25              d=25  
         C      R8,=F'3'           if m<3  
         BNL    MGE3               then  
         LA     R8,12(R8)            m=m+12  
         BCTR   R7,0                 y=y-1  
MGE3     LR     R10,R7             y  
         SRDA   R10,32             .  
         D      R10,=F'100'        r=y//100 ; l=y/100  
         LR     R3,R8              m  
         LA     R3,1(R3)           m+1  
         M      R2,=F'26'          *26  
         D      R2,=F'10'          /10  
         AR     R3,R9              +d  
         AR     R3,R10             +r  
         LR     R2,R10             r  
         SRA    R2,2               /4  
         AR     R2,R3              (d+(m+1)*26/10+r+r/4  
         LR     R3,R11             l  
         SRA    R3,2               /4  
         AR     R2,R3              (d+(m+1)*26/10+r+r/4+l/4  
         LA     R5,5               5  
         MR     R4,R11             *l  
         AR     R2,R5              (d+(m+1)*26/10+r+r/4+l/4+5*l)  
         SRDA   R2,32              .  
         D      R2,=F'7'           w=(d+(m+1)*26/10+r+r/4+l/4+5*l)//7  
         C      R2,=F'1'           if w=1  (sunday)  
         BNE    WNE1               then  
         XDECO  R6,PG                edit year  
         XPRNT  PG,12                print year  
WNE1     LA     R6,1(R6)           year=year+1  
         B      LOOP               next year  
ELOOP    BR     R14                exit  
PG       DS     CL12               buffer  
         YREGS  
         END    DOW

Output:

        2011
        2016
        2022
        2033
        2039
        2044
        2050
        2061
        2067
        2072
        2078
        2089
        2095
        2101
        2107
        2112
        2118

## [ABAP](https://rosettacode.org/wiki/Category:ABAP "Category:ABAP")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=3 "Edit section: ABAP")]

[report](http://help.sap.com/abapdocu/en/ABAPREPORT.htm) zday_of_week  
[data](http://help.sap.com/abapdocu/en/ABAPDATA.htm): lv_start type i value 2007,  
      lv_n type i value 114,  
      lv_date type sy-datum,  
      lv_weekday type string,  
      lv_day type c,  
      lv_year type n length 4.  
   
[write](http://help.sap.com/abapdocu/en/ABAPWRITE.htm) 'December 25 is a Sunday in: '.  
[do](http://help.sap.com/abapdocu/en/ABAPDO.htm) lv_n times.  
   lv_year = lv_start + sy-index.  
   [concatenate](http://help.sap.com/abapdocu/en/ABAPCONCATENATE.htm) lv_year '12' '25' into lv_date.  
   call function 'DATE_COMPUTE_DAY'  
    exporting date = lv_date  
    importing day  = lv_day.  
   
   [select](http://help.sap.com/abapdocu/en/ABAPSELECT.htm) single langt from t246 into lv_weekday  
     where sprsl = sy-langu and  
     wotnr = lv_day.  
   
   [if](http://help.sap.com/abapdocu/en/ABAPIF.htm) lv_weekday eq 'Sunday'.  
     [write](http://help.sap.com/abapdocu/en/ABAPWRITE.htm) / lv_year.  
   [endif](http://help.sap.com/abapdocu/en/ABAPENDIF.htm).  
[enddo](http://help.sap.com/abapdocu/en/ABAPENDDO.htm).  
 

Output:

December 25 is a Sunday in:
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Action!](https://rosettacode.org/wiki/Category:Action! "Category:Action!")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=4 "Edit section: Action!")]

Action! does not have a standard library providing a day of week function, therefore an adaptation of  [Sakamoto's method](https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Sakamoto.27s_methods)  to determine the day of week for a given date using integer arithmetic is used.

   
Byte FUNC DayOfWeek(BYTE day, month CARD year BYTE century)  
CARD weekday  
BYTE ARRAY index=[0 3 2 5 0 3 5 1 4 6 2 4]  
   
IF year < 100  THEN  
   year = year + century * 100   
FI               
   
IF year < 1753 THEN RETURN(7) FI  
   
IF month < 3 THEN  
   year==-1      
FI  
   
month = index(month-1)    
weekday=year + year/4 - year/100 + year/400 + month + day       
weekday = weekday MOD 7  
RETURN (weekday)  
   
PROC main()  
CARD y       
PrintE("December 25 is a Sunday in:")  
FOR y = 2008 to 2121  
DO  
IF DayOfWeek(25, 12, y)=0 THEN  
PrintCE(y)  
FI  
OD  
RETURN                                               
 

Output:

December 25 is a Sunday in:
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

  

## [Ada](https://rosettacode.org/wiki/Category:Ada "Category:Ada")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=5 "Edit section: Ada")]

with Ada.Calendar.Formatting;  use Ada.Calendar.Formatting;  
with Ada.Text_IO;              use Ada.Text_IO;  
   
procedure Yuletide is  
begin  
   for Year in 2008..2121 loop  
      if Day_Of_Week (Time_Of (Year, 12, 25)) = Sunday then  
         Put_Line (Image (Time_Of (Year, 12, 25)));  
      end if;  
   end loop;  
end Yuletide;

Output:

2011-12-25 00:00:00
2016-12-25 00:00:00
2022-12-25 00:00:00
2033-12-25 00:00:00
2039-12-25 00:00:00
2044-12-25 00:00:00
2050-12-25 00:00:00
2061-12-25 00:00:00
2067-12-25 00:00:00
2072-12-25 00:00:00
2078-12-25 00:00:00
2089-12-25 00:00:00
2095-12-25 00:00:00
2101-12-25 00:00:00
2107-12-25 00:00:00
2112-12-25 00:00:00
2118-12-25 00:00:00

## [ALGOL 68](https://rosettacode.org/wiki/Category:ALGOL_68 "Category:ALGOL 68")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=6 "Edit section: ALGOL 68")]

**Works with**:  [ALGOL 68](https://rosettacode.org/wiki/ALGOL_68 "ALGOL 68")  version Revision 1 - no extensions to language used

**Works with**:  [ALGOL 68G](https://rosettacode.org/wiki/ALGOL_68G "ALGOL 68G")  version Any - tested with release  [1.18.0-9h.tiny](https://sourceforge.net/projects/algol68/files/algol68g/algol68g-1.18.0/algol68g-1.18.0-9h.tiny.el5.centos.fc11.i386.rpm/download)

**Works with**:  [ELLA ALGOL 68](https://rosettacode.org/wiki/ELLA_ALGOL_68 "ELLA ALGOL 68")  version Any (with appropriate job cards) - tested with release  [1.8-8d](https://sourceforge.net/projects/algol68/files/algol68toc/algol68toc-1.8.8d/algol68toc-1.8-8d.fc9.i386.rpm/download)

# example from: http://www.xs4all.nl/~jmvdveer/algol.html - GPL #  
INT sun=0 # , mon=1, tue=2, wed=3, thu=4, fri=5, sat=6 #;  
   
PROC day of week = (INT year, month, day) INT: (  
  # Day of the week by Zeller’s Congruence algorithm from 1887 #  
  INT y := year, m := month, d := day, c;  
  IF m <= 2 THEN  
    m +:= 12; y -:= 1  
  FI;  
  c := y OVER 100;  
  y %*:= 100;  
  (d - 1 + ((m + 1) * 26) OVER 10 + y + y OVER 4 + c OVER 4 - 2 * c) MOD 7  
);  
   
test:(  
  print("December 25th is a Sunday in:");  
  FOR year FROM 2008 TO 2121 DO  
    INT wd = day of week(year, 12, 25);  
    IF wd = sun THEN print(whole(year,-5)) FI  
  OD;  
  new line(stand out)  
)  
 

Output:

December 25th is a Sunday in: 2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [ALGOL-M](https://rosettacode.org/wiki/Category:ALGOL-M "Category:ALGOL-M")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=7 "Edit section: ALGOL-M")]

   
BEGIN  
   
% CALCULATE P MOD Q %  
INTEGER FUNCTION MOD(P, Q);  
INTEGER P, Q;  
BEGIN  
   MOD := P - Q * (P / Q);  
END;  
   
COMMENT  
  RETURN DAY OF WEEK (SUN=0, MON=1, ETC.) FOR A GIVEN  
  GREGORIAN CALENDAR DATE USING ZELLER'S CONGRUENCE;  
INTEGER FUNCTION DAYOFWEEK(MO, DA, YR);  
INTEGER MO, DA, YR;  
BEGIN  
  INTEGER Y, C, Z;  
  IF MO < 3 THEN  
    BEGIN  
      MO := MO + 10;  
      YR := YR - 1;  
    END  
  ELSE MO := MO - 2;  
  Y := MOD(YR, 100);  
  C := YR / 100;  
  Z := (26 * MO - 2) / 10;  
  Z := Z + DA + Y + (Y / 4) + (C /4) - 2 * C + 777;  
  DAYOFWEEK := MOD(Z, 7);  
END;  
   
% MAIN PROGRAM STARTS HERE %  
INTEGER YEAR, SUNDAY;  
SUNDAY := 0;  
WRITE("CHRISTMAS WILL FALL ON A SUNDAY IN THESE YEARS:");  
FOR YEAR := 2008 STEP 1 UNTIL 2121 DO  
  BEGIN  
    IF DAYOFWEEK(12, 25, YEAR) = SUNDAY THEN  
       WRITE(YEAR);  
  END;  
   
END  
 

Output:

CHRISTMAS WILL FALL ON A SUNDAY IN THESE YEARS:
  2011
  2016
  2022
  2033
  2039
  2044
  2050
  2061
  2067
  2072
  2078
  2089
  2095
  2101
  2107
  2112
  2118

## [AppleScript](https://rosettacode.org/wiki/Category:AppleScript "Category:AppleScript")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=8 "Edit section: AppleScript")]

set ChristmasSundays to {}  
set Christmas to (current date)  
set month of Christmas to December  
set day of Christmas to 25  
repeat with year from 2008 to 2121  
	set year of Christmas to year  
	if weekday of Christmas is Sunday then set end of ChristmasSundays to year  
end repeat  
ChristmasSundays

  
Or, composing generic functions:

on run  
    -- xmasOnSunday :: Int -> Bool  
    script xmasOnSunday  
        on |λ|(y)  
            tell (current date)  
                set {its year, its month, its day, its time} to {y, 12, 25, 0}  
                return its weekday is Sunday  
            end tell  
        end |λ|  
    end script  
   
    filter(xmasOnSunday, enumFromTo(2008, 2121))  
end run  
   
   
-- GENERIC FUNCTIONS ----------------------------------------------------------  
   
-- enumFromTo :: Int -> Int -> [Int]  
on enumFromTo(m, n)  
    if m ≤ n then  
        set lst to {}  
        repeat with i from m to n  
            set end of lst to i  
        end repeat  
        return lst  
    else  
        return {}  
    end if  
end enumFromTo  
   
-- filter :: (a -> Bool) -> [a] -> [a]  
on filter(f, xs)  
    tell mReturn(f)  
        set lst to {}  
        set lng to length of xs  
        repeat with i from 1 to lng  
            set v to item i of xs  
            if |λ|(v, i, xs) then set end of lst to v  
        end repeat  
        return lst  
    end tell  
end filter  
   
-- Lift 2nd class handler function into 1st class script wrapper   
-- mReturn :: Handler -> Script  
on mReturn(f)  
    if class of f is script then  
        f  
    else  
        script  
            property |λ| : f  
        end script  
    end if  
end mReturn

Output:

{2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067,   
2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118}

## [Arc](https://rosettacode.org/wiki/Category:Arc "Category:Arc")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=9 "Edit section: Arc")]

   
(= day-names '(Sunday Monday Tuesday Wednesday Thursday Friday Saturday))  
(= get-weekday-num (fn (year month day)  
   (= helper '(0 3 2 5 0 3 5 1 4 6 2 4))  
   (if (< month 3) (= year (- year 1)))  
   (mod (+ year (helper (- month 1)) day  
        (apply + (map [trunc (/ year _)] '(4 -100 400))))  
   7)))  
(= get-weekday-name (fn (weekday-num) (day-names weekday-num)))  
 

**test:**

(up i 2008 2121  
  (when (is 0 (get-weekday-num i 12 25))  
    (prn i)))  
   
2011  
2016  
2022  
2033  
2039  
2044  
2050  
2061  
2067  
2072  
2078  
2089  
2095  
2101  
2107  
2112  
2118  
   
 

## [Arturo](https://rosettacode.org/wiki/Category:Arturo "Category:Arturo")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=10 "Edit section: Arturo")]

sundays $(filter $(range 2008 2121) { $(day &+"-12-25")="sun" })  
   
print sundays

Output:

#(2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118)

## [AutoHotkey](https://rosettacode.org/wiki/Category:AutoHotkey "Category:AutoHotkey")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=11 "Edit section: AutoHotkey")]

year = 2008  
stop = 2121  
   
While year <= stop {  
 [FormatTime](http://www.autohotkey.com/docs/commands/FormatTime.htm), day,% year 1225, dddd  
 If day = Sunday  
  out .= year "`n"  
 year++  
}  
[MsgBox](http://www.autohotkey.com/docs/commands/MsgBox.htm),% out

## [AutoIt](https://rosettacode.org/wiki/Category:AutoIt "Category:AutoIt")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=12 "Edit section: AutoIt")]

#include <date.au3>  
[Const](https://www.autoitscript.com/autoit3/docs/keywords.htm) $iSunday = 1  
[For](https://www.autoitscript.com/autoit3/docs/keywords.htm) $iYear = 2008 [To](https://www.autoitscript.com/autoit3/docs/keywords.htm) 2121 [Step](https://www.autoitscript.com/autoit3/docs/keywords.htm) 1  
   [If](https://www.autoitscript.com/autoit3/docs/keywords.htm) $iSunday = _DateToDayOfWeek($iYear, 12, 25) [Then](https://www.autoitscript.com/autoit3/docs/keywords.htm)  
     [ConsoleWrite](https://www.autoitscript.com/autoit3/docs/functions/ConsoleWrite.htm)([StringFormat](https://www.autoitscript.com/autoit3/docs/functions/StringFormat.htm)($iYear & "\n"))  
   [EndIf](https://www.autoitscript.com/autoit3/docs/keywords.htm)  
[Next](https://www.autoitscript.com/autoit3/docs/keywords.htm)

## [AWK](https://rosettacode.org/wiki/Category:AWK "Category:AWK")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=13 "Edit section: AWK")]

   
# syntax: GAWK -f DAY_OF_THE_WEEK.AWK  
# runtime does not support years > 2037 on my 32-bit Windows XP O/S  
BEGIN {  
    for (i=2008; i<=2121; i++) {  
      x = strftime("%Y/%m/%d %a",mktime(sprintf("%d 12 25 0 0 0",i)))  
      if (x ~ /Sun/) { print(x) }  
    }  
}  
 

## [BASIC](https://rosettacode.org/wiki/Category:BASIC "Category:BASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=14 "Edit section: BASIC")]

**Works with:**  FreeBASIC  
This program needs the modulo function because there is a bug in the built in modulo function.

Declare Function modulo(x As Double, y As Double) As Double  
Declare Function wd(m As Double, d As Double, y As Double) As Integer  
   
Cls  
Dim yr As Double  
For yr = 2008 To 2121  
	If wd(12,25,yr) = 1 Then  
		Print "Dec " & 25 & ", " & yr  
	EndIf  
Next  
Sleep  
   
Function modulo(x As Double, y As Double) As Double  
	If y = 0 Then  
		Return x  
	Else  
		Return x - y * Int(x / y)  
	End If  
End Function  
   
Function wd(m As Double, d As Double, y As Double) As Integer  
	If m = 1 Or m = 2 Then  
		m += 12  
		y-= 1  
	End If  
	Return modulo(365 * y + Fix(y / 4) - Fix(y / 100) + Fix(y / 400) + d  + Fix((153 * m + 8) / 5), 7) + 1  
End Function  
   
Dec 25, 2011  
Dec 25, 2016  
Dec 25, 2022  
Dec 25, 2033  
Dec 25, 2039  
Dec 25, 2044  
Dec 25, 2050  
Dec 25, 2061  
Dec 25, 2067  
Dec 25, 2072  
Dec 25, 2078  
Dec 25, 2089  
Dec 25, 2095  
Dec 25, 2101  
Dec 25, 2107  
Dec 25, 2112  
Dec 25, 2118

### [BaCon](https://rosettacode.org/wiki/Category:BaCon "Category:BaCon")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=15 "Edit section: BaCon")]

' Sunday Christmas  
PRINT "Years with Christmas on a Sunday"  
FOR y = 2008 TO 2121  
    tv = TIMEVALUE(y, 12, 25, 0, 0, 0)  
    IF WEEKDAY$(tv) = "Sunday" THEN PRINT y  
NEXT

Output:

prompt$ ./sunday-christmas
Years with Christmas on a Sunday
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

### [FreeBASIC](https://rosettacode.org/wiki/Category:FreeBASIC "Category:FreeBASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=16 "Edit section: FreeBASIC")]

' version 17-06-2015  
' compile with: fbc -s console  
   
Function wd(m As Integer, d As Integer, y As Integer) As Integer  
  If m < 3 Then        ' If m = 1 Or m = 2 Then  
    m += 12  
    y -= 1  
  End If  
  Return (y + (y \ 4) - (y \ 100) + (y \ 400) + d + ((153 * m + 8) \ 5)) Mod 7  
End Function  
   
' ------=< MAIN >=------  
   
For yr As Integer = 2008 To 2121  
  If wd(12, 25, yr) = 0 Then  
    Print "Dec 25 "; yr  
  EndIf  
Next  
   
' empty keyboard buffer   
While InKey <> "" : Wend  
Print : Print "hit any key to end program"  
Sleep  
End

Output:

Dec 25  2011
Dec 25  2016
Dec 25  2022
Dec 25  2033
Dec 25  2039
Dec 25  2044
Dec 25  2050
Dec 25  2061
Dec 25  2067
Dec 25  2072
Dec 25  2078
Dec 25  2089
Dec 25  2095
Dec 25  2101
Dec 25  2107
Dec 25  2112
Dec 25  2118

' version 17-06-2015  
' Weekday And DateSerial only works with #Include "vbcompat.bi"  
' compile with: fbc -s console  
   
#Include Once "vbcompat.bi"  
Dim As Double a  
   
For yr As Integer = 2008 To 2121  
  a = DateSerial (yr, 12, 25)  
  If Weekday(a) = 1 Then Print Format(a, "dd-mm-yyyy")   ' 1 = sunday, 2 = monday ...  
Next                                                        
   
' empty keyboard buffer   
While InKey <> "" : Wend  
Print : Print "hit any key to end program"  
Sleep  
End

Output:

25-12-2011
25-12-2016
25-12-2022
25-12-2033
25-12-2039
25-12-2044
25-12-2050
25-12-2061
25-12-2067
25-12-2072
25-12-2078
25-12-2089
25-12-2095
25-12-2101
25-12-2107
25-12-2112
25-12-2118

### [IS-BASIC](https://rosettacode.org/wiki/Category:IS-BASIC "Category:IS-BASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=17 "Edit section: IS-BASIC")]

100 PROGRAM "Dayweek.bas"  
110 PRINT "The years between 2008 and 2121 will the 25th of December be a Sunday:"  
120 FOR Y=2008 TO 2121  
130   IF DAYWEEK(Y,12,25)=0 THEN PRINT "Dec 25,";Y  
140 NEXT   
150 DEF DAYWEEK(Y,M,D)  
160   LET A=INT((14-M)/12):LET Y=Y-A  
170   LET W=D+INT((13*(M+12*A-2)-1)/5)+Y+INT(Y/4)-INT(Y/100)+INT(Y/400)  
180   LET DAYWEEK=W-7*INT(W/7)  
190 END DEF

### [Sinclair ZX81 BASIC](https://rosettacode.org/wiki/Category:Sinclair_ZX81_BASIC "Category:Sinclair ZX81 BASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=18 "Edit section: Sinclair ZX81 BASIC")]

**Translation of**:  [C](https://rosettacode.org/wiki/Day_of_the_week#C)

Works with 1k of RAM. Follows the C code quite closely: the only factors that perhaps make it less readable are (a) the absence of a modulo operator and (b) the need for continual calls to  `INT`  because we don't have an integer type. The performance is pretty acceptable; seconds rather than minutes.

 10 LET M=12  
 20 LET D=25  
 30 FOR Y=2008 TO 2121  
 40 GOSUB 80  
 50 IF W=0 THEN PRINT Y  
 60 NEXT Y  
 70 STOP  
 80 LET A=INT ((14-M)/12)  
 90 LET MM=M+12*A-2  
100 LET YY=Y-A  
110 LET W=D+INT ((13*MM-1)/5)+YY+INT (YY/4)-INT (YY/100)+INT (YY/400)  
120 LET W=W-7*INT (W/7)  
130 RETURN

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Batch File](https://rosettacode.org/wiki/Category:Batch_File "Category:Batch File")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=19 "Edit section: Batch File")]

  
:: Day of the Week task from Rosetta Code Wiki  
:: Batch File Implementation  
::  
:: In what years between 2008 and 2121 will the 25th of December be a Sunday?  
::  
:: This implementation uses Zeller's Rule...  
   
@[echo](https://www.ss64.com/nt/echo.html) off  
  
::Set month code for December  
[set](https://www.ss64.com/nt/set.html) mon=33  
  
::Set day number  
[set](https://www.ss64.com/nt/set.html) day=25  
   
[for](https://www.ss64.com/nt/for.html) /L %%w [in](https://www.ss64.com/nt/in.html) (2008,1,2121) [do](https://www.ss64.com/nt/do.html) (  
[call](https://www.ss64.com/nt/call.html) :check_day %%w  
)  
pause>[nul](https://www.ss64.com/nt/nul.html)  
[exit](https://www.ss64.com/nt/exit.html) /b  
   
:check_day  
[set](https://www.ss64.com/nt/set.html) yr=%1  
[set](https://www.ss64.com/nt/set.html) /a a=%yr%/100  
[set](https://www.ss64.com/nt/set.html) /a b=%yr%-(%a%*100)  
[set](https://www.ss64.com/nt/set.html) /a weekday=(%day%+%mon%+%b%+(%b%/4)+(%a%/4)+(5*%a%))%%7  
[if](https://www.ss64.com/nt/if.html) %weekday%==1 (  
[echo](https://www.ss64.com/nt/echo.html) Dec 25, %yr% is a Sunday.  
)  
[goto](https://www.ss64.com/nt/goto.html) :EOF  
 

Output:

Dec 25, 2011 is a Sunday.
Dec 25, 2016 is a Sunday.
Dec 25, 2022 is a Sunday.
Dec 25, 2033 is a Sunday.
Dec 25, 2039 is a Sunday.
Dec 25, 2044 is a Sunday.
Dec 25, 2050 is a Sunday.
Dec 25, 2061 is a Sunday.
Dec 25, 2067 is a Sunday.
Dec 25, 2072 is a Sunday.
Dec 25, 2078 is a Sunday.
Dec 25, 2089 is a Sunday.
Dec 25, 2095 is a Sunday.
Dec 25, 2101 is a Sunday.
Dec 25, 2107 is a Sunday.
Dec 25, 2112 is a Sunday.
Dec 25, 2118 is a Sunday.

## [BBC BASIC](https://rosettacode.org/wiki/Category:BBC_BASIC "Category:BBC BASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=20 "Edit section: BBC BASIC")]

**Works with**:  [BBC BASIC for Windows](https://rosettacode.org/wiki/BBC_BASIC_for_Windows "BBC BASIC for Windows")

      INSTALL @lib$+"DATELIB"  
   
      FOR year% = 2008 TO 2121  
        IF FN_dow(FN_mjd(25, 12, year%)) = 0 THEN  
          PRINT "Christmas Day is a Sunday in "; year%  
        ENDIF  
      NEXT

## [bc](https://rosettacode.org/wiki/Category:Bc "Category:Bc")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=21 "Edit section: bc")]

Because  _bc_  has no date library, this program uses  [_Zeller's rule_](http://mathforum.org/library/drmath/view/62324.html), also known as  [_Zeller's congruence_](http://www.merlyn.demon.co.uk/zel-like.htm), to calculate day of week.

scale = 0  
   
/*  
 * Returns day of week (0 to 6) for year, month m, day d in proleptic  
 * Gregorian calendar. Sunday is 0. Assumes y >= 1, scale = 0.  
 */  
define w(y, m, d) {  
	auto a  
   
	/* Calculate Zeller's congruence. */  
	a = (14 - m) / 12  
	m += 12 * a  
	y -= a  
	return ((d + (13 * m + 8) / 5 +			\  
		 y + y / 4 - y / 100 + y / 400) % 7)  
}  
   
for (y = 2008; y <= 2121; y++) {  
	/* If December 25 is a Sunday, print year. */  
	if (w(y, 12, 25) == 0) y  
}  
quit

## [Befunge](https://rosettacode.org/wiki/Category:Befunge "Category:Befunge")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=22 "Edit section: Befunge")]

Befunge doesn't have any standard date-handling functionality, so we calculate the day of the week directly using a simple variation of the Zeller rule.

8 >:"2("*+::::4/+\"d"/-\45v  
@_^#`"y": +1$<_v#%7+1+/*:*<  
>:#,_>$:.55+,^ >0" ,52 ceD"

Output:

Dec 25, 2011
Dec 25, 2016
Dec 25, 2022
Dec 25, 2033
Dec 25, 2039
Dec 25, 2044
Dec 25, 2050
Dec 25, 2061
Dec 25, 2067
Dec 25, 2072
Dec 25, 2078
Dec 25, 2089
Dec 25, 2095
Dec 25, 2101
Dec 25, 2107
Dec 25, 2112
Dec 25, 2118

## [Bracmat](https://rosettacode.org/wiki/Category:Bracmat "Category:Bracmat")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=23 "Edit section: Bracmat")]

**Translation of**:  [C](https://rosettacode.org/wiki/Day_of_the_week#C)

{ Calculate day of week in proleptic Gregorian calendar. Sunday == 0. }  
    ( wday  
    =   year month day adjustment mm yy  
      .   !arg:(?year,?month,?day)  
        & div$(14+-1*!month,12):?adjustment  
        & !month+12*!adjustment+-2:?mm  
        & !year+-1*!adjustment:?yy  
        &   mod  
          $ (   !day  
              + div$(13*!mm+-1,5)  
              + !yy  
              + div$(!yy,4)  
              + -1*div$(!yy,100)  
              + div$(!yy,400)  
            , 7  
            )  
    )  
& 2008:?y  
&   whl  
  ' ( !y:~>2121  
    & (   wday$(!y,12,25):0  
        & put$(str$(!y "-12-25\n"))  
      |   
      )  
    & 1+!y:?y  
    )  
& done;

Output:

2011-12-25
2016-12-25
2022-12-25
2033-12-25
2039-12-25
2044-12-25
2050-12-25
2061-12-25
2067-12-25
2072-12-25
2078-12-25
2089-12-25
2095-12-25
2101-12-25
2107-12-25
2112-12-25
2118-12-25

## [C](https://rosettacode.org/wiki/Category:C "Category:C")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=24 "Edit section: C")]

Because of problems with various C libraries (such as  _time_t_  overflowing during 2038, or strptime() or mktime() not filling in  _tm_wday_), this program uses  [Zeller's Rule](http://mathforum.org/library/drmath/view/62324.html)  to calculate day of week.

```c  
#include <stdio.h>  
   
/* Calculate day of week in proleptic Gregorian calendar. Sunday == 0. */  
int wday(int year, int month, int day)  
{  
	int adjustment, mm, yy;  
   
	adjustment = (14 - month) / 12;  
	mm = month + 12 * adjustment - 2;  
	yy = year - adjustment;  
	return (day + (13 * mm - 1) / 5 +  
		yy + yy / 4 - yy / 100 + yy / 400) % 7;  
}  
   
int main()  
{  
	int y;  
   
	for (y = 2008; y <= 2121; y++) {  
		if (wday(y, 12, 25) == 0) [printf](https://www.opengroup.org/onlinepubs/009695399/functions/printf.html)("%04d-12-25\n", y);  
	}  
   
	return 0;  
}  
```  
## [C++](https://rosettacode.org/wiki/Category:C%2B%2B "Category:C++")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=25 "Edit section: C++")]

```cpp  
#include <boost/date_time/gregorian/gregorian.hpp>  
#include <iostream>  
   
int main( ) {  
   using namespace boost::gregorian ;  
   
   std::cout  
      << "Yuletide holidays must be allowed in the following years:\n" ;  
   for ( int i = 2008 ; i < 2121 ; i++ ) {  
      greg_year gy = i ;  
      date d  ( gy, Dec , 25 ) ;  
      if ( d.day_of_week( ) == Sunday ) {  
	 std::cout << i << std::endl ;  
      }  
   }  
   std::cout << "\n" ;  
   return 0 ;  
}  
```

Output:

Yuletide holidays must be allowed in the following years:
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [C#](https://rosettacode.org/wiki/Category:C_sharp "Category:C sharp")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=26 "Edit section: C#")]

using System;  
   
class Program  
{  
    static void Main(string[] args)  
    {  
        for (int i = 2008; i <= 2121; i++)  
        {  
            DateTime date = [new](https://www.google.com/search?q=new+msdn.microsoft.com) DateTime(i, 12, 25);  
            if (date.DayOfWeek == DayOfWeek.Sunday)  
            {  
                Console.WriteLine(date.ToString("dd MMM yyyy"));  
            }  
        }  
    }  
}

Using LINQ:

using System;  
using System.Linq;  
   
class Program  
{  
    static void Main(string[] args)  
    {  
        string[] days = Enumerable.Range(2008, 2121 - 2007)  
            .Select(year => [new](https://www.google.com/search?q=new+msdn.microsoft.com) DateTime(year, 12, 25))  
            .Where(day => day.DayOfWeek == DayOfWeek.Sunday)  
            .Select(day => day.ToString("dd MMM yyyy")).ToArray();  
   
        foreach (string day in days) Console.WriteLine(day);  
    }  
}

Lambda expressions FTW:

using System;  
using System.Linq;  
   
class Program  
{  
    static void Main(string[] args)  
    {  
        Enumerable.Range(2008, 113).ToList()  
        .ConvertAll(ent => [new](https://www.google.com/search?q=new+msdn.microsoft.com) DateTime(ent, 12, 25))  
        .Where(ent => ent.DayOfWeek.Equals(DayOfWeek.Sunday)).ToList()  
        .ForEach(ent => Console.WriteLine(ent.ToString("dd MMM yyyy")));  
    }  
}

Output:

25 Dec 2011
25 Dec 2016
25 Dec 2022
25 Dec 2033
25 Dec 2039
25 Dec 2044
25 Dec 2050
25 Dec 2061
25 Dec 2067
25 Dec 2072
25 Dec 2078
25 Dec 2089
25 Dec 2095
25 Dec 2101
25 Dec 2107
25 Dec 2112
25 Dec 2118

## [Clojure](https://rosettacode.org/wiki/Category:Clojure "Category:Clojure")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=27 "Edit section: Clojure")]

Utilizing Java interop

   
(import '(java.util GregorianCalendar))  
(defn yuletide [start end]  
	    (filter #(= (. (new GregorianCalendar %  
			 (. GregorianCalendar DECEMBER) 25) get (. GregorianCalendar DAY_OF_WEEK))  
		 (. GregorianCalendar SUNDAY)) (range start (inc end))))  
   
(yuletide 2008 2121)  
 

(2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118)

## [COBOL](https://rosettacode.org/wiki/Category:COBOL "Category:COBOL")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=28 "Edit section: COBOL")]

Using Date Intrinsic Functions

   
       program-id. dec25.  
       data division.  
       working-storage section.  
       1 work-date.  
        2 yr pic 9(4) value 2008.  
        2 mo-da pic 9(4) value 1225. *> Dec 25  
       1 wk-date redefines work-date pic 9(8).  
       1 binary.  
        2 int-date pic 9(8).  
        2 dow pic 9(4).  
       procedure division.  
           perform varying yr from 2008 by 1  
           until yr > 2121  
               compute int-date = function integer-of-date (wk-date)  
               compute dow = function mod ((int-date - 1) 7) + 1  
               if dow = 7  *> Sunday = 7 per ISO 8601 and ISO 1989  
                   display yr  
               end-if  
           end-perform  
           stop run  
           .  
       end program dec25.  
 

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

Without Date Intrinsic Functions

   
       identification division.              
       program-id. dowtest.                  
       data division.                        
       working-storage section.              
       01  ws-inp-date   pic x(08).          
       01  filler redefines ws-inp-date.     
         03  ws-inp-year  pic 9(04).         
       01  ws-dow        pic 9(05).                
       procedure division.                         
           move '00001225' to ws-inp-date          
           perform test before                     
           varying ws-inp-year from 2008 by +1     
           until ws-inp-year > 2121              
             call "todow" using                    
                 by reference ws-inp-date          
                 by reference ws-dow               
                 if ws-dow = 1 then                    
                   display 'year=' ws-inp-year   
                 end-if                                
           end-perform                             
           stop run.                               
   
       end program dowtest.                        
   
       identification division.                    
       program-id.  todow.                         
       environment division.                           
       input-output section.                           
       file-control.                                   
       data division.                                  
       file section.                                   
       working-storage section.    
       01 tally pic 9(05).  
       01  wms-work-area.                              
         03  wms-year       pic 9(04).                 
         03  wms-month      pic 9(02).                 
         03  wms-csys       pic 9(01) value 1.    
         03  wms-sum        pic 9(05).  
       linkage section.                                
       01  lkip-date.                                  
         03  lkip-date-year     pic 9(04).             
         03  lkip-date-month    pic 9(02).             
         03  lkip-date-day      pic 9(02).             
       01  lkop-dow             pic 9(05).             
         88  lkop-sunday                   value 1.    
       procedure division using                        
           by reference lkip-date                      
           by reference lkop-dow                       
           .                                                              
   
           if lkip-date-month < 3                                         
             compute wms-month = lkip-date-month + 12                     
             compute wms-year  = lkip-date-year - 1                       
           else                                                           
             compute wms-month = lkip-date-month                          
             compute wms-year  = lkip-date-year                           
           end-if                                                         
   
          compute wms-sum    =                             
                          ( lkip-date-day + 2 * wms-month + wms-year      
                         + function integer (6 * (wms-month + 1) / 10)   
                         + function integer ( wms-year / 4   )           
                         - function integer ( wms-year / 100 )           
                         + function integer ( wms-year / 400 )           
                         + wms-csys )                               
         compute lkop-dow = function mod (wms-sum, 7) + 1  
                          .                                               
       end program todow.   
 

Output:

year=2011
year=2016
year=2022
year=2033
year=2039
year=2044
year=2050
year=2061
year=2067
year=2072
year=2078
year=2089
year=2095
year=2101
year=2107
year=2112
year=2118

## [CoffeeScript](https://rosettacode.org/wiki/Category:CoffeeScript "Category:CoffeeScript")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=29 "Edit section: CoffeeScript")]

december = 11 # gotta love Date APIs :)  
sunday = 0  
for year in [2008..2121]  
  xmas = new Date year, december, 25  
  console.log year if xmas.getDay() is sunday

one-liner:

console.log year for year in [2008...2121] when new Date(year, 11, 25).getDay() is 0

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [ColdFusion](https://rosettacode.org/wiki/Category:ColdFusion "Category:ColdFusion")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=30 "Edit section: ColdFusion")]

   
<cfloop from = "2008" to = "2121" index = "i">  
    <cfset myDate = createDate(i, 12, 25) />  
    <cfif dayOfWeek(myDate) eq 1>  
        December 25th falls on a Sunday in <cfoutput>#i#</cfoutput><br />  
    </cfif>  
</cfloop>  
 

## [Common Lisp](https://rosettacode.org/wiki/Category:Common_Lisp "Category:Common Lisp")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=31 "Edit section: Common Lisp")]

(loop for year from 2008 upto 2121  
   when (= 6 (multiple-value-bind  
                   (second minute hour date month year day-of-week dst-p tz)  
                 (decode-universal-time (encode-universal-time 0 0 0 25 12 year))  
               (declare (ignore second minute hour date month year dst-p tz))  
               day-of-week))  
     collect year)

(loop for year from 2008 upto 2121  
   for xmas = (encode-universal-time 0 0 0 25 12 year)  
   for day  = (nth-value 6 (decode-universal-time xmas))  
   when (= day 6) collect year)

## [Component Pascal](https://rosettacode.org/wiki/Category:Component_Pascal "Category:Component Pascal")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=32 "Edit section: Component Pascal")]

**Works with**:  [BlackBox Component Builder](https://rosettacode.org/mw/index.php?title=BlackBox_Component_Builder&action=edit&redlink=1 "BlackBox Component Builder (page does not exist)")

 ```Pascal  
MODULE DayOfWeek;  
IMPORT DevCommanders, TextMappers, Dates, StdLog;  
   
PROCEDURE XmastOnSun(s,e: INTEGER);  
VAR  
	i: INTEGER;  
	d: Dates.Date;  
BEGIN  
	i := s;d.day := 25;d.month := 12;  
	WHILE i < e DO  
		d.year := i;  
		IF Dates.DayOfWeek(d) = Dates.sunday THEN  
			StdLog.Int(i);StdLog.Ln  
		END;  
		INC(i)  
	END  
END XmastOnSun;  
   
PROCEDURE Do*;  
VAR  
	s: TextMappers.Scanner;  
	r: ARRAY 2 OF INTEGER;  
	i: INTEGER;  
BEGIN  
	s.ConnectTo(DevCommanders.par.text);  
	s.SetPos(DevCommanders.par.beg);  
	s.Scan;i := 0;  
	WHILE ~s.rider.eot DO  
		IF s.type = TextMappers.int THEN  
			r[i] := s.int; INC(i)  
		END;  
		s.Scan  
	END;  
	XmastOnSun(r[0],r[1]);  
END Do;  
   
END DayOfWeek.  
 ```  

Execute: ^Q DayOfWeek.Do 2008 2121~

Output:

 2011
 2016
 2022
 2033
 2039
 2044
 2050
 2061
 2067
 2072
 2078
 2089
 2095
 2101
 2107
 2112
 2118

## [D](https://rosettacode.org/wiki/Category:D "Category:D")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=33 "Edit section: D")]

void main() {  
    import std.stdio, std.range, std.algorithm, std.datetime;  
   
    writeln("Christmas comes on a Sunday in the years:\n",  
            iota(2008, 2122)  
            .filter!(y => Date(y, 12, 25).dayOfWeek == DayOfWeek.sun));  
}

Output:

Christmas comes on a Sunday in the years:
[2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

## [Delphi](https://rosettacode.org/wiki/Category:Delphi "Category:Delphi")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=34 "Edit section: Delphi")]

**Library:**  [sysutils](https://rosettacode.org/mw/index.php?title=Category:Sysutils&action=edit&redlink=1 "Category:Sysutils (page does not exist)")

always in uses clause in Delphi

procedure IsXmasSunday(fromyear, toyear: integer);  
var  
i: integer;  
TestDate: TDateTime;  
outputyears: string;  
begin  
outputyears := '';  
  for i:= fromyear to toyear do  
  begin  
    TestDate := EncodeDate(i,12,25);  
    if dayofweek(TestDate) = 1 then  
    begin  
      outputyears := outputyears + inttostr(i) + ' ';  
    end;  
  end;  
  //CONSOLE  
  //writeln(outputyears);  
  //GUI   
  form1.label1.caption := outputyears;  
end;

Procedure called with year range to test and outputs a space-delimited array of years to a label. There is no error check that fromyear < toyear, but this is easily added.

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [ECL](https://rosettacode.org/wiki/Category:ECL "Category:ECL")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=35 "Edit section: ECL")]

//In what years between 2008 and 2121 will the 25th of December be a Sunday?  
   
IMPORT STD;  
   
BaseYear := 2008;  
EndYear  := 2121;  
   
ChristmasDay := RECORD  
  UNSIGNED1 DayofWeek;  
  UNSIGNED2 Year;  
END;  
   
ChristmasDay FindDate(INTEGER Ctr) := TRANSFORM  
  SELF.DayofWeek := (STD.Date.FromGregorianYMD((BaseYear-1) + Ctr,12,25)) % 7; //0=Sunday  
  SELF.Year := (BaseYear-1) + Ctr;  
END;  
   
YearDS := DATASET(EndYear-BaseYear,FindDate(COUNTER));  
OUTPUT(YearDS(DayofWeek=0),{Year});  
   
/* Outputs:   
   2011  
   2016  
   2022  
   2033  
   2039  
   2044  
   2050  
   2061  
   2067  
   2072  
   2078  
   2089  
   2095  
   2101  
   2107  
   2112  
   2118  
*/  
 

This code solves a specific task, but can easily be modified as a generic function to return the DayOfWeek for any day after 1 AD.

## [Elixir](https://rosettacode.org/wiki/Category:Elixir "Category:Elixir")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=36 "Edit section: Elixir")]

**Works with**:  [Elixir](https://rosettacode.org/wiki/Elixir "Elixir")  version 1.4

Enum.each(2008..2121, fn year ->  
  wday = Date.from_erl!({year, 12, 25}) |> Date.day_of_week  
  if wday==7, do: IO.puts "25 December #{year} is sunday"  
end)

Output:

25 December 2011 is sunday
25 December 2016 is sunday
25 December 2022 is sunday
25 December 2033 is sunday
25 December 2039 is sunday
25 December 2044 is sunday
25 December 2050 is sunday
25 December 2061 is sunday
25 December 2067 is sunday
25 December 2072 is sunday
25 December 2078 is sunday
25 December 2089 is sunday
25 December 2095 is sunday
25 December 2101 is sunday
25 December 2107 is sunday
25 December 2112 is sunday
25 December 2118 is sunday

## [Erlang](https://rosettacode.org/wiki/Category:Erlang "Category:Erlang")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=37 "Edit section: Erlang")]

% Implemented by bengt kleberg  
-module(yuletide).  
-export([main/0, sunday_years/2]).  
   
main() ->  
	[[io](http://erlang.org/doc/man/io.html):fwrite("25 December ~p is Sunday~n", [X]) || X <- sunday_years(2008, 2121)].  
   
sunday_years( Start, Stop ) ->  
	[X || X <- [lists](http://erlang.org/doc/man/lists.html):seq(Start, Stop), is_sunday([calendar](http://erlang.org/doc/man/calendar.html):day_of_the_week({X, 12, 25}))].  
   
is_sunday( 7 ) -> true;  
is_sunday( _ ) -> false.

Output:

25 December 2011 is Sunday
25 December 2016 is Sunday
25 December 2022 is Sunday
25 December 2033 is Sunday
25 December 2039 is Sunday
25 December 2044 is Sunday
25 December 2050 is Sunday
25 December 2061 is Sunday
25 December 2067 is Sunday
25 December 2072 is Sunday
25 December 2078 is Sunday
25 December 2089 is Sunday
25 December 2095 is Sunday
25 December 2101 is Sunday
25 December 2107 is Sunday
25 December 2112 is Sunday
25 December 2118 is Sunday

## [ERRE](https://rosettacode.org/wiki/Category:ERRE "Category:ERRE")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=38 "Edit section: ERRE")]

   
PROGRAM DAY_OF_THE_WEEK  
   
PROCEDURE MODULO(X,Y->RES)  
   IF Y=0 THEN  
      RES=X  
     ELSE  
      RES=X-Y*INT(X/Y)  
   END IF  
END PROCEDURE  
   
PROCEDURE WD(M,D,Y->RES%)  
   IF M=1 OR M=2 THEN  
     M+=12  
     Y-=1  
   END IF  
   MODULO(365*Y+INT(Y/4)-INT(Y/100)+INT(Y/400)+D+INT((153*M+8)/5),7->RES)  
   RES%=RES+1.0  
END PROCEDURE  
   
BEGIN  
PRINT(CHR$(12);) ! CLS  
FOR YR=2008 TO 2121 DO  
    WD(12,25,YR->RES%)  
    IF RES%=1 THEN  ! day 1 is Sunday......  
      PRINT("Dec";25;",";YR)  
    END IF  
END FOR  
GET(K$)  
END PROGRAM  
 

Output:

Dec 25, 2011
Dec 25, 2016
Dec 25, 2022
Dec 25, 2033
Dec 25, 2039
Dec 25, 2044
Dec 25, 2050
Dec 25, 2061
Dec 25, 2067
Dec 25, 2072
Dec 25, 2078
Dec 25, 2089
Dec 25, 2095
Dec 25, 2101
Dec 25, 2107
Dec 25, 2112
Dec 25, 2118

## [Euphoria](https://rosettacode.org/wiki/Category:Euphoria "Category:Euphoria")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=39 "Edit section: Euphoria")]

   
--Day of the week task from Rosetta Code wiki  
--User:Lnettnay  
   
--In what years between 2008 and 2121 will the 25th of December be a Sunday  
   
include std/datetime.e  
   
datetime dt  
   
for year = 2008 to 2121 do  
        dt = new(year, 12, 25)  
        if weeks_day(dt) = 1 then -- Sunday = 1  
                ? year  
        end if  
end for  
 

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

  

## [F#](https://rosettacode.org/wiki/Category:F_Sharp "Category:F Sharp")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=40 "Edit section: F#")]

open System  
   
[ 2008 .. 2121 ]  
|> [List](http://research.microsoft.com/en-us/um/cambridge/projects/fsharp/manual/namespaces.html).choose (fun y -> if DateTime(y,12,25).DayOfWeek = DayOfWeek.Sunday then Some(y) else None)  
|> printfn "%A"

Output:

[2011; 2016; 2022; 2033; 2039; 2044; 2050; 2061; 2067; 2072; 2078; 2089; 2095;
 2101; 2107; 2112; 2118]

## [Factor](https://rosettacode.org/wiki/Category:Factor "Category:Factor")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=41 "Edit section: Factor")]

USING: calendar math.ranges prettyprint sequences ;  
2008 2121 [a,b] [ 12 25 <date> sunday? ] filter .

## [FBSL](https://rosettacode.org/wiki/Category:FBSL "Category:FBSL")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=42 "Edit section: FBSL")]

#APPTYPE CONSOLE  
   
'In what years between 2008 and 2121 will the 25th of December be a Sunday?  
[DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) date [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [INTEGER](http://www.qbasicnews.com/qboho/qckinteger.shtml), dayname [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [STRING](http://www.qbasicnews.com/qboho/qckstring.shtml)  
FOR [DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) year = 2008 TO 2121  
	date = year * 10000 + 1225  
	dayname = dateconv(date,"dddd")  
	IF dayname = "Sunday" THEN  
		[PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) "Christmas Day is on a Sunday in ", year  
	[END](http://www.qbasicnews.com/qboho/qckend.shtml) IF  
NEXT  
PAUSE  
 

## [Fōrmulæ](https://rosettacode.org/wiki/Category:F%C5%8Drmul%C3%A6 "Category:Fōrmulæ")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=43 "Edit section: Fōrmulæ")]

In  [this](https://wiki.formulae.org/Day_of_the_week)  page you can see the solution of this task.

Fōrmulæ programs are not textual, visualization/edition of programs is done showing/manipulating structures but not text ([more info](http://wiki.formulae.org/Editing_F%C5%8Drmul%C3%A6_expressions)). Moreover, there can be multiple visual representations of the same program. Even though it is possible to have textual representation —i.e. XML, JSON— they are intended for transportation effects more than visualization and edition.

The option to show Fōrmulæ programs and their results is showing images. Unfortunately images cannot be uploaded in Rosetta Code.

## [Forth](https://rosettacode.org/wiki/Category:Forth "Category:Forth")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=44 "Edit section: Forth")]

Forth has only TIME&DATE, which does not give day of week. Many public Forth Julian date calculators had year-2100 problems, but this algorithm works well.

   
\ Zeller's Congruence  
: weekday ( d m y -- wd) \ 1 mon..7 sun  
  over 3 < if 1- swap 12 + swap then  
  100 /mod  
  dup 4 / swap 2* -  
  swap dup 4 / + +  
  swap 1+ 13 5 */ + +  
  ( in zeller 0=sat, so -2 to 0= mon, then mod, then 1+ for 1=mon)  
  2- 7 mod 1+ ;    
   
: yuletide  
  ." December 25 is Sunday in "  
  2122 2008 do  
    25 12 i weekday  
    7 = if i . then  
  loop cr ;   
 

   
cr yuletide  
December 25 is Sunday in 2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118  
 ok  
 

  
To show year-2100 problems with SwiftForth's provided Modified Julian Day support:

: yuletide  
  ." December 25 is Sunday in "  
  2122 2008 do  
    25 12 i d/m/y  
    7 mod 0= if i . then  
  loop cr ;  
   
cr yuletide  
December 25 is Sunday in 2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2100 2106 2117

In  [4tH](https://rosettacode.org/wiki/4tH "4tH")  a library is available which provides the right answer:

include lib/time.4th  
   
: yuletide  
  ." December 25 is Sunday in "  
  2122 2008 do  
    25 12 i weekday  
    6 = if i . then  
  loop cr ;  
   
cr yuletide

The code is derived from "Collected Algorithms from ACM", Volume 1 Algorithms 1-220.

## [Fortran](https://rosettacode.org/wiki/Category:Fortran "Category:Fortran")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=45 "Edit section: Fortran")]

**Works with**:  [Fortran](https://rosettacode.org/wiki/Fortran "Fortran")  version 90 and later

Based on Forth example

PROGRAM YULETIDE  
   
IMPLICIT NONE  
   
INTEGER :: day, year  
   
WRITE(*, "(A)", ADVANCE="NO") "25th of December is a Sunday in"  
DO year = 2008, 2121  
   day = Day_of_week(25, 12, year)  
   IF (day == 1) WRITE(*, "(I5)", ADVANCE="NO") year  
END DO  
   
CONTAINS  
   
FUNCTION Day_of_week(d, m, y)  
   INTEGER :: Day_of_week, j, k, mm, yy  
   INTEGER, INTENT(IN) :: d, m, y  
   
   mm=m  
   yy=y  
   IF(mm.le.2) THEN  
      mm=mm+12  
      yy=yy-1  
   END IF  
   j = yy / 100  
   k = MOD(yy, 100)  
   Day_of_week = MOD(d + ((mm+1)*26)/10 + k + k/4 + j/4 + 5*j, 7)  
END FUNCTION Day_of_week  
   
END PROGRAM YULETIDE

Output:

 25th of December is a Sunday in 2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Gambas](https://rosettacode.org/wiki/Category:Gambas "Category:Gambas")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=46 "Edit section: Gambas")]

**[Click this link to run this code](https://gambas-playground.proko.eu/?gist=b9b4e9a871e96ea6f1db467fa23669fe)**

[Public](http://gambasdoc.org/help/lang/public) [Sub](http://gambasdoc.org/help/lang/sub) Main()  
[Dim](http://gambasdoc.org/help/lang/dim) siCount [As](http://gambasdoc.org/help/lang/as) [Short](http://gambasdoc.org/help/lang/type/short)  
   
[For](http://gambasdoc.org/help/lang/for) siCount = 2008 [To](http://gambasdoc.org/help/lang/to) 2121  
  [If](http://gambasdoc.org/help/lang/if) [WeekDay](http://gambasdoc.org/help/lang/weekday)([Date](http://gambasdoc.org/help/lang/type/date)(siCount, 12, 25)) = 0 [Then](http://gambasdoc.org/help/lang/then) [Print](http://gambasdoc.org/help/lang/print) [Format](http://gambasdoc.org/help/lang/format)([Date](http://gambasdoc.org/help/lang/type/date)(siCount, 12, 25), "dddd dd mmmm yyyy") & " falls on a Sunday"  
[Next](http://gambasdoc.org/help/lang/next)  
   
[End](http://gambasdoc.org/help/lang/end)

Output:

Sunday 25 December 2011 falls on a Sunday
Sunday 25 December 2016 falls on a Sunday
Sunday 25 December 2022 falls on a Sunday
Sunday 25 December 2033 falls on a Sunday
Sunday 25 December 2039 falls on a Sunday
Sunday 25 December 2044 falls on a Sunday
Sunday 25 December 2050 falls on a Sunday
Sunday 25 December 2061 falls on a Sunday
Sunday 25 December 2067 falls on a Sunday
Sunday 25 December 2072 falls on a Sunday
Sunday 25 December 2078 falls on a Sunday
Sunday 25 December 2089 falls on a Sunday
Sunday 25 December 2095 falls on a Sunday
Sunday 25 December 2101 falls on a Sunday
Sunday 25 December 2107 falls on a Sunday
Sunday 25 December 2112 falls on a Sunday
Sunday 25 December 2118 falls on a Sunday

## [GAP](https://rosettacode.org/wiki/Category:GAP "Category:GAP")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=47 "Edit section: GAP")]

Filtered([2008 .. 2121], y -> WeekDay([25, 12, y]) = "Sun");  
# [ 2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118 ]  
   
# A possible implementation of WeekDayAlt  
   
days := ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];;  
   
WeekDayAlt := function(args)  
   local d, m, y, k;  
   d := args[1];  
   m := args[2];  
   y := args[3];  
   if m < 3 then  
      m := m + 12;  
      y := y - 1;  
   fi;  
   k := 1 + RemInt(d + QuoInt((m + 1)*26, 10) + y + QuoInt(y, 4)  
          + 6*QuoInt(y, 100) + QuoInt(y, 400) + 5, 7);  
   return days[k];  
end;  
   
Filtered([2008 .. 2121], y -> WeekDayAlt([25, 12, y]) = "Sun");  
# [ 2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118 ]

## [Go](https://rosettacode.org/wiki/Category:Go "Category:Go")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=48 "Edit section: Go")]

package main  
   
import "fmt"  
import "time"  
   
func main() {  
    for year := 2008; year <= 2121; year++ {  
        if time.Date(year, 12, 25, 0, 0, 0, 0, time.UTC).Weekday() ==  
            time.Sunday {  
            fmt.Printf("25 December %d is Sunday\n", year)  
        }  
    }  
}

Output:

25 December 2011 is Sunday
25 December 2016 is Sunday
25 December 2022 is Sunday
25 December 2033 is Sunday
25 December 2039 is Sunday
25 December 2044 is Sunday
25 December 2050 is Sunday
25 December 2061 is Sunday
25 December 2067 is Sunday
25 December 2072 is Sunday
25 December 2078 is Sunday
25 December 2089 is Sunday
25 December 2095 is Sunday
25 December 2101 is Sunday
25 December 2107 is Sunday
25 December 2112 is Sunday
25 December 2118 is Sunday

## [Groovy](https://rosettacode.org/wiki/Category:Groovy "Category:Groovy")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=49 "Edit section: Groovy")]

Solution:

[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) yuletide = { [start](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20start), stop -> ([start](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20start)..stop).[findAll](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20findAll) { [Date](https://www.google.de/search?as_q=Date&num=100&hl=en&as_occt=url&as_sitesearch=java.sun.com%2Fj2se%2F1%2E5%2E0%2Fdocs%2Fapi%2F).parse("yyyy-MM-dd", "${it}-12-25").format("EEE") == "Sun" } }

Test program:

[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) yuletide(2008, 2121)

Output:

[2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

## [Haskell](https://rosettacode.org/wiki/Category:Haskell "Category:Haskell")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=50 "Edit section: Haskell")]

Using the time library:

import Data.Time (fromGregorian)  
   
import Data.Time.Calendar.WeekDate (toWeekDate)  
   
isXmasSunday :: [Integer](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:Integer) -> [Bool](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:Bool)  
isXmasSunday year =  
  let (_, _, wday) = toWeekDate $ fromGregorian year 12 25  
  in wday == 7  
   
main :: [IO](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:IO) ()  
main =  
  [mapM_](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:mapM_)  
    [putStrLn](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:putStrLn)  
    [ "25 December " ++ [show](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:show) year ++ " is Sunday"  
    | year <- [2008 .. 2121]   
    , isXmasSunday year ]

Output:

25 December 2011 is Sunday
25 December 2016 is Sunday
25 December 2022 is Sunday
25 December 2033 is Sunday
25 December 2039 is Sunday
25 December 2044 is Sunday
25 December 2050 is Sunday
25 December 2061 is Sunday
25 December 2067 is Sunday
25 December 2072 is Sunday
25 December 2078 is Sunday
25 December 2089 is Sunday
25 December 2095 is Sunday
25 December 2101 is Sunday
25 December 2107 is Sunday
25 December 2112 is Sunday
25 December 2118 is Sunday

The built-in System.Time module can overflow at the Unix epoch in 2038:

import System.Time  
   
isXmasSunday :: [Int](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:Int) -> [Bool](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:Bool)  
isXmasSunday year = ctWDay cal == Sunday  
  where  
    cal = toUTCTime $ toClockTime cal'  
    cal' =  
      CalendarTime  
      { ctYear = year  
      , ctMonth = December  
      , ctDay = 25  
      , ctHour = 0  
      , ctMin = 0  
      , ctSec = 0  
      , ctPicosec = 0  
      , ctWDay = Friday  
      , ctYDay = 0  
      , ctTZName = ""  
      , ctTZ = 0  
      , ctIsDST = False  
      }  
   
main :: [IO](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:IO) ()  
main =  
  [mapM_](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:mapM_)  
    [putStrLn](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:putStrLn)  
    [ "25 December " ++ [show](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:show) year ++ " is Sunday"  
    | year <- [2008 .. 2121]   
    , isXmasSunday year ]

Output:

on 32-bit machine:

25 December 2011 is Sunday
25 December 2016 is Sunday
25 December 2022 is Sunday
25 December 2033 is Sunday
*** Exception: user error (Time.toClockTime: invalid input)

but with 64 bit systems, running current versions of GHC:

25 December 2011 is Sunday
25 December 2016 is Sunday
25 December 2022 is Sunday
25 December 2033 is Sunday
25 December 2039 is Sunday
25 December 2044 is Sunday
25 December 2050 is Sunday
25 December 2061 is Sunday
25 December 2067 is Sunday
25 December 2072 is Sunday
25 December 2078 is Sunday
25 December 2089 is Sunday
25 December 2095 is Sunday
25 December 2101 is Sunday
25 December 2107 is Sunday
25 December 2112 is Sunday
25 December 2118 is Sunday

## [HicEst](https://rosettacode.org/wiki/Category:HicEst "Category:HicEst")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=51 "Edit section: HicEst")]

DO year = 1, 1000000  
   TIME(Year=year, MOnth=12, Day=25, TO, WeekDay=weekday)  
   IF( weekday == 7) WRITE(StatusBar) year  
ENDDO  
   
END

No anomalies detected for the first million years :-)
Dec 25 = Sunday in 
5 ... 2011 2016 2022 2033 2039 2044 2050 2061 2067
      2072 2078 2089 2095 2101 2107 2112 2118 ... 999994

## [Icon](https://rosettacode.org/wiki/Category:Icon "Category:Icon")  and  [Unicon](https://rosettacode.org/wiki/Category:Unicon "Category:Unicon")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=52 "Edit section: Icon and Unicon")]

link datetime  
   
procedure main()  
writes("December 25th is a Sunday in: ")  
every writes((dayoweek(25,12,y := 2008 to 2122)=="Sunday",y)," ")  
end

  

**Library:**  [Icon Programming Library](https://rosettacode.org/wiki/Category:Icon_Programming_Library "Category:Icon Programming Library")

[datetime provides dayoweek](https://www.cs.arizona.edu/icon/library/src/procs/datetime.icn)

procedure dayoweek(day, month, year)	#: day of the week  
   static d_code, c_code, m_code, ml_code, y, C, M, Y  
   
   initial {  
      d_code := ["Saturday", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]  
   
      c_code := table()  
      c_code[16] := c_code[20] := 0  
      c_code[17] := c_code[21] := 6  
      c_code[18] := c_code[22] := 4  
      c_code[19] := c_code[23] := 2  
   
      m_code := table()  
      m_code[1] := m_code["January"] := 1  
      m_code[2] := m_code["February"] := 4  
      m_code[3] := m_code["March"] := 4  
      m_code[4] := m_code["April"] := 0  
      m_code[5] := m_code["May"] := 2  
      m_code[6] := m_code["June"] := 5  
      m_code[7] := m_code["July"] := 0  
      m_code[8] := m_code["August"] := 3  
      m_code[9] := m_code["September"] := 6  
      m_code[10] := m_code["October"] := 1  
      m_code[11] := m_code["November"] := 4  
      m_code[12] := m_code["December"] := 6  
   
      ml_code := copy(m_code)  
      ml_code[1] := ml_code["January"] := 0  
      ml_code[2] := ml_code["February"] := 3  
      }  
   
   if year < 1600 then stop("*** can't compute day of week that far back")  
   if year > 2299 then stop("*** can't compute day of week that far ahead")  
   
   C := c_code[(year / 100) + 1]  
   y := year % 100  
   Y := (y / 12) + (y % 12) + ((y % 12) / 4)  
   month := integer(month)  
   M := if (year % 4) = 0 then ml_code[month] else m_code[month]  
   
   return d_code[(C + Y + M + day) % 7 + 1]   
   
end

Output:

December 25th is a Sunday in: 2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [J](https://rosettacode.org/wiki/Category:J "Category:J")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=53 "Edit section: J")]

   load 'dates'                                    NB. provides verb 'weekday'  
   xmasSunday=: #~ 0 = [: weekday 12 25 ,~"1 0 ]   NB. returns years where 25 Dec is a Sunday  
   xmasSunday 2008 + i.114                         NB. check years from 2008 to 2121  
2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Java](https://rosettacode.org/wiki/Category:Java "Category:Java")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=54 "Edit section: Java")]

import java.util.Calendar;  
import java.util.Date;  
import java.util.GregorianCalendar;  
   
public class Yuletide{  
	public static void main([String](https://www.google.com/search?hl=en&q=allinurl%3Astring+java.sun.com&btnI=I%27m%20Feeling%20Lucky)[] args) {  
		for(int i = 2008;i<=2121;i++){  
			[Calendar](https://www.google.com/search?hl=en&q=allinurl%3Acalendar+java.sun.com&btnI=I%27m%20Feeling%20Lucky) cal = new [GregorianCalendar](https://www.google.com/search?hl=en&q=allinurl%3Agregoriancalendar+java.sun.com&btnI=I%27m%20Feeling%20Lucky)(i, [Calendar](https://www.google.com/search?hl=en&q=allinurl%3Acalendar+java.sun.com&btnI=I%27m%20Feeling%20Lucky).DECEMBER,  
					25);  
			if(cal.get([Calendar](https://www.google.com/search?hl=en&q=allinurl%3Acalendar+java.sun.com&btnI=I%27m%20Feeling%20Lucky).DAY_OF_WEEK)==[Calendar](https://www.google.com/search?hl=en&q=allinurl%3Acalendar+java.sun.com&btnI=I%27m%20Feeling%20Lucky).SUNDAY){  
				[System](https://www.google.com/search?hl=en&q=allinurl%3Asystem+java.sun.com&btnI=I%27m%20Feeling%20Lucky).out.println(cal.getTime());  
			}  
		}  
	}  
}

Output:

Sun Dec 25 00:00:00 CST 2011
Sun Dec 25 00:00:00 CST 2016
Sun Dec 25 00:00:00 CST 2022
Sun Dec 25 00:00:00 CST 2033
Sun Dec 25 00:00:00 CST 2039
Sun Dec 25 00:00:00 CST 2044
Sun Dec 25 00:00:00 CST 2050
Sun Dec 25 00:00:00 CST 2061
Sun Dec 25 00:00:00 CST 2067
Sun Dec 25 00:00:00 CST 2072
Sun Dec 25 00:00:00 CST 2078
Sun Dec 25 00:00:00 CST 2089
Sun Dec 25 00:00:00 CST 2095
Sun Dec 25 00:00:00 CST 2101
Sun Dec 25 00:00:00 CST 2107
Sun Dec 25 00:00:00 CST 2112
Sun Dec 25 00:00:00 CST 2118

## [JavaScript](https://rosettacode.org/wiki/Category:JavaScript "Category:JavaScript")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=55 "Edit section: JavaScript")]

### ES5[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=56 "Edit section: ES5")]

#### Iteration[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=57 "Edit section: Iteration")]

for (var year = 2008; year <= 2121; year++){  
    var xmas = new Date(year, 11, 25)  
    if ( xmas.getDay() === 0 )  
        console.log(year)  
}

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

#### Functional composition[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=58 "Edit section: Functional composition")]

(function () {  
    'use strict';  
   
    // isXmasSunday :: Integer -> Bool  
    function isXmasSunday(year) {  
        return (new Date(year, 11, 25))  
            .getDay() === 0;  
    }  
   
    // range :: Int -> Int -> [Int]  
    function range(m, n) {  
        return Array.apply(null, Array(n - m + 1))  
            .map(function (_, i) {  
                return m + i;  
            });  
    }  
   
    return range(2008, 2121)  
        .filter(isXmasSunday);  
   
})();

Output:

[2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 
2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

### ES6[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=59 "Edit section: ES6")]

(() => {  
    'use strict';  
   
    // main :: IO ()  
    const main = () => {  
        const  
            xs = enumFromTo(2008, 2121)  
            .filter(xmasIsSunday);  
        return (  
            console.log(xs),  
            xs  
        );  
    };  
   
   
    // xmasIsSunday :: Int -> Bool  
    const xmasIsSunday = year =>  
        (new Date(year, 11, 25))  
        .getDay() === 0;  
   
   
    // enumFromTo :: Int -> Int -> [Int]  
    const enumFromTo = (m, n) =>  
        Array.from({  
            length: 1 + n - m  
        }, (_, i) => m + i);  
   
   
    return main();  
})();

Output:

[2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

## [jq](https://rosettacode.org/wiki/Category:Jq "Category:Jq")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=60 "Edit section: jq")]

# Use Zeller's Congruence to determine the day of the week, given  
# year, month and day as integers in the conventional way.  
# If iso == "iso" or "ISO", then emit an integer in 1 -- 7 where   
# 1 represents Monday, 2 Tuesday, etc;  
# otherwise emit 0 for Saturday, 1 for Sunday, etc.  
#  
def day_of_week(year; month; day; iso):  
  if month == 1 or month == 2 then  
    [month + 12, year - 1]  
  else  
    [month, year]  
  end   
  | day + (13*(.[0] + 1)/5|floor)  
    +  (.[1]%100)       + ((.[1]%100)/4|floor)  
    +  (.[1]/400|floor) - 2*(.[1]/100|floor)   
  | if iso == "iso" or iso == "ISO" then 1 + ((. + 5) % 7)  
    else . % 7  
    end;

**The task**:

# Give the results as an array so they can  
# readily be presented on a single line:  
[range(2008; 2122) | select( day_of_week(.;12;25;0) == 1 )]

Output:

$ jq -n -c -f zeller.jq
[2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118]

## [Jsish](https://rosettacode.org/wiki/Category:Jsish "Category:Jsish")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=61 "Edit section: Jsish")]

Jsi does not yet implement the Javascript  _Date_  object.  _strftime' and_ strptime _functions are used here instead._

/* Day of the week, December 25th on a Sunday */  
for (var year = 2008; year <= 2121; year++) {  
    var xmas = strptime(year + '/12/25', '%Y/%m/%d');  
    var weekDay = strftime(xmas, '%w');  
    if (weekDay == 0) puts(year);  
}  
   
/*  
=!EXPECTSTART!=  
2011  
2016  
2022  
2033  
2039  
2044  
2050  
2061  
2067  
2072  
2078  
2089  
2095  
2101  
2107  
2112  
2118  
=!EXPECTEND!=  
*/

Output:

prompt$ jsish -u dayOfTheWeek.jsi
[PASS] dayOfTheWeek.jsi

## [Julia](https://rosettacode.org/wiki/Category:Julia "Category:Julia")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=62 "Edit section: Julia")]

using Dates  
   
lo, hi = 2008, 2121  
xmas = collect(Date(lo, 12, 25):Year(1):Date(hi, 12, 25))  
filter!(xmas) do dt  
    dayofweek(dt) == Dates.Sunday  
end  
   
println("Years from $lo to $hi having Christmas on Sunday: ")  
foreach(println, year.(xmas))

Output:

Years from 2008 to 2121 having Christmas on Sunday: 
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [K](https://rosettacode.org/wiki/Category:K "Category:K")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=63 "Edit section: K")]

    wd:{(__jd x)!7}  / Julian day count, Sun=6  
    y@&6={wd 1225+x*10000}'y:2008+!114  
2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118  
 

## [Kotlin](https://rosettacode.org/wiki/Category:Kotlin "Category:Kotlin")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=64 "Edit section: Kotlin")]

// version 1.0.6  
   
[import](https://scala-lang.org/) java.util.*  
   
fun main(args: Array<String>) {  
    println("Christmas day in the following years falls on a Sunday:\n")  
    [val](https://scala-lang.org/) calendar = GregorianCalendar(2008, Calendar.DECEMBER, 25)  
    [for](https://scala-lang.org/) (year in 2008..2121) {  
        [if](https://scala-lang.org/) (Calendar.SUNDAY == calendar[Calendar.DAY_OF_WEEK]) println(year)  
        calendar.add(Calendar.YEAR, 1)  
    }  
}

Output:

Christmas day in the following years falls on a Sunday:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Lasso](https://rosettacode.org/wiki/Category:Lasso "Category:Lasso")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=65 "Edit section: Lasso")]

loop(-From=2008, -to=2121) => {^  
  local(tDate = date('12/25/' + loop_count))  
  #tDate->dayOfWeek == 1 ? '\r' + #tDate->format('%D') + ' is a Sunday'  
^}

Output:

12/25/2011 is a Sunday
12/25/2016 is a Sunday
12/25/2022 is a Sunday
12/25/2033 is a Sunday
12/25/2039 is a Sunday
12/25/2044 is a Sunday
12/25/2050 is a Sunday
12/25/2061 is a Sunday
12/25/2067 is a Sunday
12/25/2072 is a Sunday
12/25/2078 is a Sunday
12/25/2089 is a Sunday
12/25/2095 is a Sunday
12/25/2101 is a Sunday
12/25/2107 is a Sunday
12/25/2112 is a Sunday
12/25/2118 is a Sunday

## [Liberty BASIC](https://rosettacode.org/wiki/Category:Liberty_BASIC "Category:Liberty BASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=66 "Edit section: Liberty BASIC")]

    count = 0  
    for year = 2008 to 2121  
        dateString$="12/25/";year  
        dayNumber=date$(dateString$)  
   
        if dayNumber mod 7 = 5 then  
            count = count + 1  
            print dateString$  
        end if  
   
    next year  
   
    print count; " years when Christmas Day falls on a Sunday"  
    end

## [Lingo](https://rosettacode.org/wiki/Category:Lingo "Category:Lingo")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=67 "Edit section: Lingo")]

put "December 25 is a Sunday in:"  
refDateObj = date(1905,1,2)  
repeat with year = 2008 to 2121  
  dateObj = date(year, 12, 25)  
  dayOfWeek = ((dateObj - refDateObj) mod 7)+1 -- 1=Monday..7=Sunday  
  if dayOfWeek=7 then put year  
end repeat

Output:

-- "December 25 is a Sunday in:"
-- 2011
-- 2016
-- 2022
-- 2033
-- 2039
-- 2044
-- 2050
-- 2061
-- 2067
-- 2072
-- 2078
-- 2089
-- 2095
-- 2101
-- 2107
-- 2112
-- 2118

## [LiveCode](https://rosettacode.org/wiki/Category:LiveCode "Category:LiveCode")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=68 "Edit section: LiveCode")]

function xmasSunday startDate endDate  
    convert the long date to dateitems  
    put it into xmasDay  
    put 12 into item 2 of xmasDay  
    put 25 into item 3 of xmasDay  
    repeat with i = startDate to endDate  
        put i into item 1 of xmasDay  
        convert xmasDay to dateItems  
        if item 7 of xmasDay is 1 then put i & comma after xmasYear  
    end repeat  
    if the last char of xmasYear is comma then delete the last char of xmasYear  
    return xmasYear  
end xmasSunday

Example

put xmasSunday(2008,2121)

Output

2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118

## [Logo](https://rosettacode.org/wiki/Category:Logo "Category:Logo")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=69 "Edit section: Logo")]

; Determine if a Gregorian calendar year is leap   
to leap? :year  
  output (and   
    equal? 0 modulo :year 4  
    not member? modulo :year 400 [100 200 300]  
  )  
end  
   
; Convert Gregorian calendar date to a simple day count from   
; day 1 = January 1, 1 CE   
to day_number :year :month :day  
  local "elapsed make "elapsed difference :year 1  
  output (sum  product 365 :elapsed  
              int quotient :elapsed 4  
              minus int quotient :elapsed 100  
              int quotient :elapsed 400  
              int quotient difference product 367 :month 362 12  
              ifelse lessequal? :month 2 0 ifelse leap? :year -1 -2  
              :day)  
end  
   
; Find the day of the week from a day number; 0 = Sunday through 6 = Saturday  
to day_of_week :day_number  
  output modulo :day_number 7  
end  
   
; True if the given day is a Sunday  
to sunday? :year :month :day  
  output equal? 0 day_of_week day_number :year :month :day  
end  
   
; Put it all together to answer the question posed in the problem  
print filter [sunday? ? 12 25] iseq 2008 2121  
bye

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Lua](https://rosettacode.org/wiki/Category:Lua "Category:Lua")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=70 "Edit section: Lua")]

**Library:**  [LuaDate](http://luaforge.net/projects/date/)

require("date")  
   
for year=2008,2121 do  
   if date(year, 12, 25):getweekday() == 1 then  
      print(year)  
   end  
end

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

### Without external modules[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=71 "Edit section: Without external modules")]

Same output as above

local dTab = {day = 25, month = 12}  
for year = 2008, 2121 do  
    dTab.year = year  
    if os.date("%A", os.time(dTab)) == "Sunday" then  
        print(year)  
    end  
end

## [M2000 Interpreter](https://rosettacode.org/wiki/Category:M2000_Interpreter "Category:M2000 Interpreter")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=72 "Edit section: M2000 Interpreter")]

Str$( number, format$) use Visual Basic 6 format

   
Print "December 25 is a Sunday in:"  
For Year=2008 to 2121 {   
      if  Str$(Date("25/12/"+str$(Year,"")),"w")="1" Then {  
            Print Year  
      }  
}  
\\ is the same with this:  
Print "December 25 is a Sunday in:"  
For Year=2008 to 2121 {   
      if  Str$(Date(str$(Year,"")+"-12-25"),"w")="1" Then {  
            Print Year  
      }  
}  
   
   
 

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [M4](https://rosettacode.org/wiki/Category:M4 "Category:M4")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=73 "Edit section: M4")]

divert(-1)  
   
define(`for',  
   `ifelse($#,0,``$0'',  
   `ifelse(eval($2<=$3),1,  
   `pushdef(`$1',$2)$4`'popdef(`$1')$0(`$1',incr($2),$3,`$4')')')')  
   
dnl  julian day number corresponding to December 25th of given year  
define(`julianxmas',  
   `define(`yrssince0',eval($1+4712))`'define(`noOfLpYrs',  
      eval((yrssince0+3)/4))`'define(`jd',  
      eval(365*yrssince0+noOfLpYrs-10-($1-1501)/100+($1-1201)/400+334+25-1))`'  
      ifelse(eval($1%4==0 && ($1%100!=0 || $1%400==0)),1,  
         `define(`jd',incr(jd))')`'jd')  
   
divert  
   
for(`yr',2008,2121,  
   `ifelse(eval(julianxmas(yr)%7==6),1,`yr ')')

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112
2118

## [Maple](https://rosettacode.org/wiki/Category:Maple "Category:Maple")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=74 "Edit section: Maple")]

xmas:= proc()  
	local i, dt;  
	for i from 2008 to 2121 by 1 do  
		dt := Date(i, 12, 25);  
		if (Calendar:-DayOfWeek(dt) = 1) then  
			print(i);  
		end if;  
	end do;  
end proc;  
   
xmas();

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

  

## [Mathematica](https://rosettacode.org/wiki/Category:Mathematica "Category:Mathematica")  /  [Wolfram Language](https://rosettacode.org/wiki/Category:Wolfram_Language "Category:Wolfram Language")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=75 "Edit section: Mathematica / Wolfram Language")]

Reap[If[DateString[{#,12,25},"DayName"]=="Sunday",Sow[#]]&/@Range[2008,2121]][[2,1]]

gives back:

{2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118}

## [MATLAB](https://rosettacode.org/wiki/Category:MATLAB "Category:MATLAB")  /  [Octave](https://rosettacode.org/wiki/Category:Octave "Category:Octave")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=76 "Edit section: MATLAB / Octave")]

  t  = [datenum](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/datenum.html)([[2008:2121]',[repmat](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/repmat.html)([12,25,0,0,0], 2121-2007, 1)]);  
  t  = t([strmatch](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/strmatch.html)('Sunday', [datestr](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/datestr.html)(t,'dddd')), :);  
  [datestr](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/datestr.html)(t,'yyyy')		  
 

  

Output:

 ans =
  2011
  2016
  2022
  2033
  2039
  2044
  2050
  2061
  2067
  2072
  2078
  2089
  2095
  2101
  2107
  2112
  2118

## [Maxima](https://rosettacode.org/wiki/Category:Maxima "Category:Maxima")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=77 "Edit section: Maxima")]

weekday(year, month, day) := block([m: month, y: year, k],  
   if m < 3 then (m: m + 12, y: y - 1),  
   k: 1 + remainder(day + quotient((m + 1)*26, 10) + y + quotient(y, 4)  
        + 6*quotient(y, 100) + quotient(y, 400) + 5, 7),  
   ['monday, 'tuesday, 'wednesday, 'thurdsday, 'friday, 'saturday, 'sunday][k]  
)$  
   
sublist(makelist(i, i, 2008, 2121),  
        lambda([y], weekday(y, 12, 25) = 'sunday));  
/* [2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118] */

## [Modula-3](https://rosettacode.org/wiki/Category:Modula-3 "Category:Modula-3")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=78 "Edit section: Modula-3")]

**Translation of**:  [C](https://rosettacode.org/wiki/Day_of_the_week#C)

Modula-3 represents time using a (safe) wrapper around the C time interface. Consequently, it suffers from the same problem as C.

MODULE Yule EXPORTS Main;  
   
IMPORT IO, Fmt, Date, Time;  
   
VAR date: Date.T;  
    time: Time.T;  
   
BEGIN  
  FOR year := 2008 TO 2121 DO  
    date.day := 25;  
    date.month := Date.Month.Dec;  
    date.year := year;  
   
    TRY  
      time := Date.ToTime(date);  
    EXCEPT  
    | Date.Error =>   
      IO.Put(Fmt.Int(year) & " is the last year we can specify\n");  
      EXIT;  
    END;  
   
    date := Date.FromTime(time);  
   
    IF date.weekDay = Date.WeekDay.Sun THEN  
      IO.Put("25th of December " & Fmt.Int(year) & " is Sunday\n");  
    END;  
  END;  
END Yule.

Output:

25th of December 2011 is Sunday
25th of December 2016 is Sunday
25th of December 2022 is Sunday
25th of December 2033 is Sunday
2038 is the last year we can specify

## [МК-61/52](https://rosettacode.org/wiki/Category:%D0%9C%D0%9A-61/52 "Category:МК-61/52")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=79 "Edit section: МК-61/52")]

П9	7	П7	1	П8	НОП	ИП8	2	2	-  
1	0	/	[x]	П6	ИП9	+	1	8	9  
9	-	3	6	5	,	2	5	*	[x]  
ИП8	ИП6	1	2	*	-	1	4	-	3  
0	,	5	9	*	[x]	+	2	9	+  
ИП7	+	П4	ИП4	7	/	[x]	7	*	-  
x=0	64	ИП9	С/П	ИП9	1	+	П9	БП	06

_Input:_  РX: starting year.

_Output:_  the year in which Christmas falls on a Sunday. For example, enter  _2008_, the first result:  _2018_  (_January 7, 2018_  is Sunday).

## [MUMPS](https://rosettacode.org/wiki/Category:MUMPS "Category:MUMPS")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=80 "Edit section: MUMPS")]

**Library:**  [VA Kernel](https://rosettacode.org/mw/index.php?title=Category:VA_Kernel&action=edit&redlink=1 "Category:VA Kernel (page does not exist)") **version**  22.0

   
DOWHOLIDAY  
 ;In what years between 2008 and 2121 will December 25 be a Sunday?  
 ;Uses the VA's public domain routine %DTC (Part of the Kernel) named here DIDTC  
 NEW BDT,EDT,CHECK,CHKFOR,LIST,I,X,Y  
 ;BDT - the beginning year to check  
 ;EDT - the end year to check  
 ;BDT and EDT are year offsets from the epoch date 1/1/1700  
 ;CHECK - the month and day to look at  
 ;CHKFOR - what day of the week to look for  
 ;LIST - list of years in which the condition is true  
 ;I - the year currently being checked  
 ;X - the date in an "internal" format, for input to DOW^DIDTC  
 ;Y - the output from DOW^DIDTC  
 SET BDT=308,EDT=421,CHECK="1225",CHKFOR=0,LIST=""  
 FOR I=BDT:1:EDT SET X=I_CHECK D DOW^DIDTC SET:(Y=0) LIST=$SELECT($LENGTH(LIST):LIST_", ",1:"")_(I+1700)  
 IF $LENGTH(LIST)=0 WRITE !,"There are no years that have Christmas on a Sunday in the given range."  
 IF $LENGTH(LIST) WRITE !,"The following years have Christmas on a Sunday: ",LIST  
 KILL BDT,EDT,CHECK,CHKFOR,LIST,I,X,Y  
 QUIT  
 

Usage:

USER>D ^DOW

The following years have Christmas on a Sunday: 2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118

## [NetRexx](https://rosettacode.org/wiki/Category:NetRexx "Category:NetRexx")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=81 "Edit section: NetRexx")]

/* NetRexx */  
options replace format comments java crossref savelog symbols nobinary  
   
yearRanges = [int 2008, 2121]  
searchday = ''  
cal = Calendar  
   
loop year = yearRanges[0] to yearRanges[1]  
  cal = GregorianCalendar(year, Calendar.DECEMBER, 25)  
  dayIndex = cal.get(Calendar.DAY_OF_WEEK)  
  if dayIndex = Calendar.SUNDAY then searchday = searchday year  
  end year  
   
say 'Between' yearRanges[0] 'and' yearRanges[1]', Christmas day falls on a Sunday on the following years:'  
searchday = searchday.strip.changestr(' ', ',')  
say '  'searchday  
   
return  
 

Output:

Between 2008 and 2121, Christmas day falls on a Sunday on the following years:
  2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118

### Comparison of Some Common Day-of-Week Algorithms[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=82 "Edit section: Comparison of Some Common Day-of-Week Algorithms")]

The following program exercises some common "Day-0f-Week" algorithms to confirm they all arrive at the same result.

/* NetRexx */  
options replace format comments java crossref savelog symbols nobinary  
   
days = 'Monday Tuesday Wednesday Thursday Friday Saturday Sunday'  
yearRanges = [int 2008, 2121]  
   
searchday = ''  
searchday['index'] = days.wordpos('Sunday')  
searchday[0] = 0  
   
algorithmName = ['Java Calendar', 'Zeller[1]', 'Zeller[2]', 'Sakamoto', 'Gauss', 'Keith', 'Babwani']  
   
loop alg = 0 to algorithmName.length - 1  
  sd = searchday[0] + 1  
  searchday[0] = sd  
  searchday['agorithm', sd] = algorithmName[alg]  
  loop year = yearRanges[0] to yearRanges[1]  
    select case alg  
      when 0 then dayIndex = getDaynumJavaLibrary(year, 12, 25)  
      when 1 then dayIndex = getDaynumZellersCongruenceMethod1(year, 12, 25)  
      when 2 then dayIndex = getDaynumZellersCongruenceMethod2(year, 12, 25)  
      when 3 then dayIndex = getDaynumSakamoto(year, 12, 25)  
      when 4 then dayIndex = getDaynumGauss(year, 12, 25)  
      when 5 then dayIndex = getDaynumKeith(year, 12, 25)  
      when 6 then dayIndex = getDaynumBabwani(year, 12, 25)  
      otherwise nop  
      end  
    if dayIndex = searchday['index'] then  
      searchday[sd] = searchday[sd] year  
    end year  
  end alg  
   
-- display results  
say 'Between' yearRanges[0] 'and' yearRanges[1]', Christmas day falls on a Sunday in the following years:'  
loop r_ = 1 to searchday[0]  
  searchday[r_] = searchday[r_].strip.changestr(' ', ',')  
  say searchday['agorithm', r_].right(20)':' searchday[r_]  
  end r_  
   
return  
   
-- -----------------------------------------------------------------------------  
method getDaynumJavaLibrary(Year = int, Month = int, Day = int, iso = Rexx 'Y') public static binary returns int  
  -- The day-of-week is an integer value where 1 is Sunday, 2 is Monday, ..., and 7 is Saturday  
  -- For an ISO week date Day-of-Week d (1 = Monday to 7 = Sunday), use d = ((h - 1 + 6) mod 7) + 1  
   
  cal = Calendar  
  jmNumber = [ -  
      Calendar.JANUARY,   Calendar.FEBRUARY, Calendar.MARCH,    Calendar.APRIL    -  
    , Calendar.MAY,       Calendar.JUNE,     Calendar.JULY,     Calendar.AUGUST   -  
    , Calendar.SEPTEMBER, Calendar.OCTOBER,  Calendar.NOVEMBER, Calendar.DECEMBER -  
    ]  
   
  mon = jmNumber[Month - 1]  
  cal = GregorianCalendar(Year, mon, Day)  
  h   = cal.get(Calendar.DAY_OF_WEEK)  
   
  if 'YES'.abbrev(iso.upper, 1) then w = ((h - 1 + 6) // 7) + 1  
                                else w = h  
   
  return w  
   
-- -----------------------------------------------------------------------------  
method getDaynumZellersCongruenceMethod1(Year = int, Month = int, Day = int, iso = Rexx 'Y') public static returns int  
  -- DayNum results in an integer in the range 0-6 where 0 represents Monday etc.  
  -- For an ISO week date add 1  
   
  if Month = 1 | Month = 2 then do  
    Month = Month + 12  
    Year  = Year - 1  
    end  
   
  MonthFactor = 2 * Month + 3 * (Month + 1) % 5  
  YearFactor  = Year + Year % 4 - Year % 100 + Year % 400  
  DayNum      = (Day + MonthFactor + YearFactor) // 7  
   
  if 'YES'.abbrev(iso.upper, 1) then d = DayNum + 1  
                                else d = DayNum  
   
  return d  
   
-- -----------------------------------------------------------------------------  
method getDaynumZellersCongruenceMethod2(Year = int, Month = int, Day = int, iso = Rexx 'Y') public static binary returns int  
  -- h is the day of the week (0 = Saturday, 1 = Sunday, 2 = Monday, ...)  
  -- For an ISO week date Day-of-Week d (1 = Monday to 7 = Sunday), use d = ((h + 5) mod 7) + 1  
   
  if Month < 3 then do  
    Month = Month + 12  
    Year  = Year - 1  
    end  
  q = Day  
  m = Month  
  Y = Year  
   
  h = (q + ((m + 1) * 26 % 10) + Y + (Y % 4) + 6 * (Y % 100) + (Y % 400)) // 7  
   
  if 'YES'.abbrev(iso.upper, 1) then d = ((h + 5) // 7) + 1  
                                else d = h  
   
  return d  
   
-- -----------------------------------------------------------------------------  
method getDaynumSakamoto(y = int, m = int, d = int, iso = Rexx 'Y') public static binary returns int  
  -- h is the day of the week (0 = Sunday, 1 = Monday, 2 = Tuesday...)  
  -- For an ISO week date Day-of-Week d (1 = Monday to 7 = Sunday), use d = ((h + 6) mod 7) + 1  
   
  t = [int 0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4]  
  y = y - (m < 3)  
  h = (y + y % 4 - y % 100 + y % 400 + t[m - 1] + d) // 7  
   
  if 'YES'.abbrev(iso.upper, 1) then d = ((h + 6) // 7) + 1  
                                else d = h  
   
  return d  
   
-- -----------------------------------------------------------------------------  
method getDaynumGauss(Year = int, Month = int, Day = int, iso = Rexx 'Y') public static binary returns int  
  -- W is week day (0 = Sunday, ..., 6 = Saturday)  
  -- For an ISO week date Day-of-Week d (1 = Monday to 7 = Sunday), use d = ((h + 6) mod 7) + 1  
   
  Year = Year - (Month < 3)  
  k = double Day  
  C = double Year % 100  
  Y = double Year // 100  
  m = double ((Month + 9) // 12) + 1  
   
  W = modulo(int (k + Math.floor(2.6 * m - 0.2) + y + Math.floor(y / 4) + Math.floor(c / 4) - 2 * c), 7)  
   
  if 'YES'.abbrev(iso.upper, 1) then h = ((W + 6) // 7) + 1  
                                else h = W  
   
  return h  
   
-- -----------------------------------------------------------------------------  
method getDaynumKeith(y = int, m = int, d = int, iso = Rexx 'Y') public constant binary returns int  
  -- W is week day (0 = Sunday, ..., 6 = Saturday)  
  -- For an ISO week date Day-of-Week d (1 = Monday to 7 = Sunday), use d = ((h + 6) mod 7) + 1  
   
  if m < 3 then do  
    d = d + y  
    y = y - 1  
    end  
  else do  
    d = d + y - 2  
    end  
   
  h = (23 * m % 9 + d + 4 + y % 4 - y % 100 + y % 400) // 7  
   
  if 'YES'.abbrev(iso.upper, 1) then W = ((h + 6) // 7) + 1  
                                else W = h  
   
  return W  
   
-- -----------------------------------------------------------------------------  
method getDaynumBabwani(Year = int, Month = int, Day = int, iso = Rexx 'Y') public constant binary returns int  
  -- return dow = Day of week: 0 = Saturday, 1 = Sunday, ... 6 = Friday  
  -- For an ISO week date Day-of-Week W (1 = Monday to 7 = Sunday), use W = ((dow + 5) mod 7) + 1  
   
  y = Year  
  m = Month  
  d = Day  
   
  dow    = int -- dow stands for day of week  
  dowfg  = double  
  fmonth = int  
  leap   = int  
   
  if ((y // 100 == 0) & (y // 400 \= 0)) then  -- leap function 1 for leap & 0 for non-leap  
    leap = 0  
  else if (y // 4 == 0) then  
    leap = 1  
  else  
    leap = 0  
   
  fmonth = 3 + (2 - leap) * ((m + 2) % (2 * m)) + (5 * m + m % 9) % 2 -- f(m) formula  
  fmonth = fmonth // 7 -- f(m) is brought in range of 0 to 6  
   
  century    = y % 100  
  lastdigits = y // 100  
   
  dowfg = 1.25 * lastdigits + fmonth + d - 2 * (century // 4) -- function of weekday for Gregorian  
  dow = int dowfg // 7 -- remainder on division by 7  
   
  if 'YES'.abbrev(iso.upper, 1) then W = ((dow + 5) // 7) + 1  
                                else W = dow  
   
  return W  
   
-- -----------------------------------------------------------------------------  
method modulo(N = int, D = int) inheritable static binary returns int  
  return (D + (N // D)) // D  
 

Output:

Between 2008 and 2121, Christmas day falls on a Sunday in the following years:
       Java Calendar: 2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118
           Zeller[1]: 2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118
           Zeller[2]: 2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118
            Sakamoto: 2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118
               Gauss: 2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118
               Keith: 2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118
             Babwani: 2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118

## [Nim](https://rosettacode.org/wiki/Category:Nim "Category:Nim")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=83 "Edit section: Nim")]

import times  
   
var timeinfo = getLocalTime getTime()  
timeinfo.monthday = 25  
timeinfo.month = mDec  
for year in 2008..2121:  
  timeinfo.year = year  
  if getLocalTime(timeInfoToTime timeinfo).weekday == dSun:  
    stdout.write year," "

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118 

## [Oberon-2](https://rosettacode.org/wiki/Category:Oberon-2 "Category:Oberon-2")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=84 "Edit section: Oberon-2")]

**Works with**:  [oo2c version 2](https://rosettacode.org/mw/index.php?title=Oo2c_version_2&action=edit&redlink=1 "Oo2c version 2 (page does not exist)")

   
MODULE DayOfWeek;  
IMPORT NPCT:Dates, Out;  
VAR  
  year: INTEGER;  
  date: Dates.Date;  
BEGIN  
  FOR year := 2008 TO 2121 DO  
    date := Dates.NewDate(25,12,year);  
    IF date.DayOfWeek() = Dates.sunday THEN  
     Out.Int(date.year,4);Out.Ln  
    END  
  END  
END DayOfWeek.  
 

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

**Works with**:  [AOS](https://rosettacode.org/mw/index.php?title=AOS&action=edit&redlink=1 "AOS (page does not exist)")

   
MODULE DaysOfWeek; (** AUTHOR ""; PURPOSE ""; *)  
   
IMPORT  
	Out := KernelLog, Dates;  
   
PROCEDURE Do*;  
VAR  
	date: Dates.DateTime;  
	i,y,w,wd: LONGINT;  
BEGIN  
	FOR i := 2008 TO 2121 DO  
		date.year := i;date.month :=12; date.day := 25;  
		date.hour := 0;date.minute := 0; date.second := 0;  
		Dates.WeekDate(date,y,w,wd);  
		IF wd = 7 THEN Out.Int(i,0);Out.Ln END  
	END  
END Do;  
   
END DaysOfWeek.  
 

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Objective-C](https://rosettacode.org/wiki/Category:Objective-C "Category:Objective-C")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=85 "Edit section: Objective-C")]

**Works with**:  [GNUstep](https://rosettacode.org/wiki/GNUstep "GNUstep")

**Works with**:  [Cocoa](https://rosettacode.org/wiki/Cocoa "Cocoa")

#import <Foundation/Foundation.h>  
   
int main()  
{  
   @autoreleasepool {  
      for(NSUInteger i=2008; i<2121; i++)  
      {  
         [NSCalendarDate](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSCalendarDate_Class/) *d = [[[NSCalendarDate](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSCalendarDate_Class/) alloc]   
                              initWithYear: i  
                              month: 12  
                              day: 25  
                              hour: 0 minute: 0 second:0   
                              timeZone: [[NSTimeZone](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSTimeZone_Class/) timeZoneWithAbbreviation:@"CET"] ];  
         if ( [d dayOfWeek] == 0 )  
         {    
            [printf](https://www.opengroup.org/onlinepubs/009695399/functions/printf.html)("25 Dec %u is Sunday\n", i);  
         }  
      }  
   
   }  
   return 0;  
}

Output:

25 Dec 2011 is Sunday
25 Dec 2016 is Sunday
25 Dec 2022 is Sunday
25 Dec 2033 is Sunday
25 Dec 2039 is Sunday
25 Dec 2044 is Sunday
25 Dec 2050 is Sunday
25 Dec 2061 is Sunday
25 Dec 2067 is Sunday
25 Dec 2072 is Sunday
25 Dec 2078 is Sunday
25 Dec 2089 is Sunday
25 Dec 2095 is Sunday
25 Dec 2101 is Sunday
25 Dec 2107 is Sunday
25 Dec 2112 is Sunday
25 Dec 2118 is Sunday

## [OCaml](https://rosettacode.org/wiki/Category:OCaml "Category:OCaml")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=86 "Edit section: OCaml")]

**Translation of**:  [C](https://rosettacode.org/wiki/Day_of_the_week#C)

#load "unix.cma"  
open [Unix](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Unix.html)  
   
try  
  for i = 2008 to 2121 do  
    (* I'm lazy so we'll just borrow the current time  
       instead of having to set all the fields explicitly *)  
    let mytime = { (localtime (time ())) with  
                   tm_year  = i - 1900;  
                   tm_mon   = 11;  
                   tm_mday  = 25 } in  
    try  
      let _, mytime = mktime mytime in  
        if mytime.tm_wday = 0 then  
          [Printf](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Printf.html).printf "25 December %d is Sunday\n" i  
    with e ->  
      [Printf](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Printf.html).printf "%d is the last year we can specify\n" (i-1);  
      raise e  
  done  
with _ -> ()

Output:

of a run on a 32 bit machine

25 December 2011 is Sunday
25 December 2016 is Sunday
25 December 2022 is Sunday
25 December 2033 is Sunday
2037 is the last year we can specify

### With a dedicated library[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=87 "Edit section: With a dedicated library")]

**Library:**  [OCaml Calendar Library](https://rosettacode.org/wiki/Category:OCaml_Calendar_Library "Category:OCaml Calendar Library")

Unlike the previous example which only uses the OCaml standard library, here with the OCaml Calendar Library we can go until the year 2121:

open CalendarLib  
   
let list_make_seq first last =  
  let rec aux i acc =  
    if i < first then acc  
    else aux ([pred](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#VALpred) i) (i::acc)  
  in  
  aux last []  
   
let print_date (year, month, day) =  
  [Printf](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Printf.html).printf "%d-%02d-%02d\n" year month day  
   
let () =  
  let years = list_make_seq 2008 2121 in  
  let years = [List](http://caml.inria.fr/pub/docs/manual-ocaml/libref/List.html).filter (fun year ->  
    Date.day_of_week (Date.make year 12 25) = Date.Sun) years in  
  [print_endline](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#VALprint_endline) "December 25 is a Sunday in:";  
  [List](http://caml.inria.fr/pub/docs/manual-ocaml/libref/List.html).iter ([Printf](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Printf.html).printf "%d\n") years

Output:

$ ocaml unix.cma str.cma -I +calendar calendarLib.cma xmas_sundays.ml
December 25 is a Sunday in:
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Oforth](https://rosettacode.org/wiki/Category:Oforth "Category:Oforth")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=88 "Edit section: Oforth")]

import: date  
seqFrom(2008, 2121) filter(#[ 12 25 Date newDate dayOfWeek Date.SUNDAY == ]) .

Output:

[2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

## [ooRexx](https://rosettacode.org/wiki/Category:OoRexx "Category:OoRexx")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=89 "Edit section: ooRexx")]

date = .datetime~new(2008, 12, 25)  
lastdate = .datetime~new(2121, 12, 25)  
   
resultList = .array~new -- our collector of years  
   
-- date objects are directly comparable  
loop while date <= lastdate  
  if date~weekday == 7 then resultList~append(date~year)  
  -- step to the next year  
  date = date~addYears(1)  
end  
   
say "Christmas falls on Sunday in the years" resultList~toString("Line", ", ")

Output:

Christmas falls on Sunday in the years 2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118

## [PARI/GP](https://rosettacode.org/wiki/Category:PARI/GP "Category:PARI/GP")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=90 "Edit section: PARI/GP")]

   
njd(D) =  
{  
  my (m, y);  
   
  if (D[2] > 2, y = D[1]; m = D[2]+1, y = D[1]-1; m = D[2]+13);  
   
  (1461*y)\4 + (306001*m)\10000 + D[3] - 694024 + if (100*(100*D[1]+D[2])+D[3] > 15821004, 2 - y\100 + y\400)  
}  
   
for (y = 2008, 2121, if (njd([y,12,25]) % 7 == 1, print(y)));  
 

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Pascal](https://rosettacode.org/wiki/Category:Pascal "Category:Pascal")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=91 "Edit section: Pascal")]

**Library:**  [sysutils](https://rosettacode.org/mw/index.php?title=Category:Sysutils&action=edit&redlink=1 "Category:Sysutils (page does not exist)")

**Works with**:  [Free Pascal](https://rosettacode.org/wiki/Free_Pascal "Free Pascal")

See  [Delphi](https://rosettacode.org/wiki/Day_of_the_week#Delphi "Day of the week")

## [Peloton](https://rosettacode.org/wiki/Category:Peloton "Category:Peloton")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=92 "Edit section: Peloton")]

<@ SAI>  
	<@ ITEFORLI3>2121|2008|  
		<@ LETVARCAP>Christmas Day|25-Dec-<@ SAYVALFOR>...</@></@>  
		<@ TSTDOWVARLIT>Christmas Day|1</@>  
		<@ IFF>  
			<@ SAYCAP>Christmas Day <@ SAYVALFOR>...</@> is a Sunday</@><@ SAYKEY>__Newline</@>  
		</@>  
	</@>  
</@>

English dialect variable-length space-padded opcodes

<# suppressimplicitoutput>  
	<# iterate foriteration literalstring3>2121|2008|  
		<# let variable capture>Christmas Day|25-Dec-<# say value foriteration>...</#></#>  
		<# test dayofweek variable literal>Christmas Day|1</#>  
		<# if>  
			<# say capture>Christmas Day <# say value foriteration>...</#> is a Sunday</#><# say keyword>__Newline</#>  
		</#>  
	</#>  
   
</#>

Output:

Christmas Day 2011 is a Sunday
Christmas Day 2016 is a Sunday
Christmas Day 2022 is a Sunday
Christmas Day 2033 is a Sunday
Christmas Day 2039 is a Sunday
Christmas Day 2044 is a Sunday
Christmas Day 2050 is a Sunday
Christmas Day 2061 is a Sunday
Christmas Day 2067 is a Sunday
Christmas Day 2072 is a Sunday
Christmas Day 2078 is a Sunday
Christmas Day 2089 is a Sunday
Christmas Day 2095 is a Sunday
Christmas Day 2101 is a Sunday
Christmas Day 2107 is a Sunday
Christmas Day 2112 is a Sunday
Christmas Day 2118 is a Sunday

## [Perl](https://rosettacode.org/wiki/Category:Perl "Category:Perl")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=93 "Edit section: Perl")]

#! /usr/bin/perl -w  
   
use Time::Local;  
use strict;  
   
foreach my $i (2008 .. 2121)  
{  
  my $time = timelocal(0,0,0,25,11,$i);  
  my ($s,$m,$h,$md,$mon,$y,$wd,$yd,$is) = [localtime](https://perldoc.perl.org/functions/localtime.html)($time);  
  if ( $wd == 0 )  
  {  
    [print](https://perldoc.perl.org/functions/print.html) "25 Dec $i is Sunday\n";  
  }  
}  
   
[exit](https://perldoc.perl.org/functions/exit.html) 0;

Output:

25 Dec 2011 is Sunday
25 Dec 2016 is Sunday
25 Dec 2022 is Sunday
25 Dec 2033 is Sunday
Day too big - 25195 > 24855
Sec too small - 25195 < 78352
Sec too big - 25195 > 15247
Cannot handle date (0, 0, 0, 25, 11, 2038) at ./ydate.pl line 8

Using the DateTime module from CPAN:

#! /usr/bin/perl -w  
   
use DateTime;  
use strict;  
   
foreach my $i (2008 .. 2121)  
{  
  my $dt = DateTime->new( year   => $i,  
                          month  => 12,  
                          day    => 25  
                        );  
  if ( $dt->day_of_week == 7 )  
  {  
    [print](https://perldoc.perl.org/functions/print.html) "25 Dec $i is Sunday\n";  
  }  
}  
   
[exit](https://perldoc.perl.org/functions/exit.html) 0;

or shorter:

#! /usr/bin/perl -w  
   
use DateTime;  
use strict;  
   
for (2008 .. 2121) {  
  [print](https://perldoc.perl.org/functions/print.html) "25 Dec $_ is Sunday\n"  
    if DateTime->new(year => $_, month => 12, day => 25)->day_of_week == 7;  
}  
   
[exit](https://perldoc.perl.org/functions/exit.html) 0;

Output:

25 Dec 2011 is Sunday
25 Dec 2016 is Sunday
25 Dec 2022 is Sunday
25 Dec 2033 is Sunday
25 Dec 2039 is Sunday
25 Dec 2044 is Sunday
25 Dec 2050 is Sunday
25 Dec 2061 is Sunday
25 Dec 2067 is Sunday
25 Dec 2072 is Sunday
25 Dec 2078 is Sunday
25 Dec 2089 is Sunday
25 Dec 2095 is Sunday
25 Dec 2101 is Sunday
25 Dec 2107 is Sunday
25 Dec 2112 is Sunday
25 Dec 2118 is Sunday

Alternatively in one line using grep (read from right to left):

#! /usr/bin/perl -w  
   
use DateTime;  
use strict;  
   
[print](https://perldoc.perl.org/functions/print.html) [join](https://perldoc.perl.org/functions/join.html) " ", [grep](https://perldoc.perl.org/functions/grep.html) { DateTime->new(year => $_, month => 12, day => 25)->day_of_week == 7 } (2008 .. 2121);  
   
0;

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Perl 6](https://rosettacode.org/wiki/Category:Perl_6 "Category:Perl 6")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=94 "Edit section: Perl 6")]

**Works with**:  [Rakudo](https://rosettacode.org/wiki/Rakudo "Rakudo")  version 2010.07

As Perl 5, except  `DateTime`  is built-in, so you don't need to download a module of that name:

say join ' ', grep { Date.new($_, 12, 25).day-of-week == 7 }, 2008 .. 2121;

## [Phix](https://rosettacode.org/wiki/Category:Phix "Category:Phix")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=95 "Edit section: Phix")]

sequence res = {}  
for y=2008 to 2121 do  
    if day_of_week(y,12,25)=1 then  
        res = append(res,y)  
    end if  
end for  
?res

Output:

{2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118}

## [PHP](https://rosettacode.org/wiki/Category:PHP "Category:PHP")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=96 "Edit section: PHP")]

<?php  
for($i=2008; $i<2121; $i++)  
{  
  $datetime = new DateTime("$i-12-25 00:00:00");  
  if ( $datetime->format("w") == 0 )  
  {  
     echo "25 Dec $i is Sunday\n";  
  }  
}  
?>

Output:

25 Dec 2011 is Sunday
25 Dec 2016 is Sunday
25 Dec 2022 is Sunday
25 Dec 2033 is Sunday
25 Dec 2039 is Sunday
25 Dec 2044 is Sunday
25 Dec 2050 is Sunday
25 Dec 2061 is Sunday
25 Dec 2067 is Sunday
25 Dec 2072 is Sunday
25 Dec 2078 is Sunday
25 Dec 2089 is Sunday
25 Dec 2095 is Sunday
25 Dec 2101 is Sunday
25 Dec 2107 is Sunday
25 Dec 2112 is Sunday
25 Dec 2118 is Sunday

## [PicoLisp](https://rosettacode.org/wiki/Category:PicoLisp "Category:PicoLisp")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=97 "Edit section: PicoLisp")]

(for (Y 2008 (>= 2121 Y) (inc Y))  
   (when (= "Sunday" (day (date Y 12 25)))  
      (printsp Y) ) )

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Pike](https://rosettacode.org/wiki/Category:Pike "Category:Pike")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=98 "Edit section: Pike")]

filter(Calendar.Year(2008)->range(Calendar.Year(2121))->years()->month(12)->day(25), lambda(object day){ return day->week_day()==7; })->year()->format_nice();

Output:

 Result: ({ /* 17 elements */
                 "2011",
                 "2016",
                 "2022",
                 "2033",
                 "2039",
                 "2044",
                 "2050",
                 "2061",
                 "2067",
                 "2072",
                 "2078",
                 "2089",
                 "2095",
                 "2101",
                 "2107",
                 "2112",
                 "2118"
             })

## [PL/I](https://rosettacode.org/wiki/Category:PL/I "Category:PL/I")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=99 "Edit section: PL/I")]

   
declare i picture '9999';  
do i = 2008 to 2121;  
   if weekday(days('25Dec' || i, 'DDMmmYYYY')) = 1 then  
      put skip list ('Christmas day ' || i || ' is a Sunday');  
end;  
 

## [PowerShell](https://rosettacode.org/wiki/Category:PowerShell "Category:PowerShell")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=100 "Edit section: PowerShell")]

2008..2121 | Where-Object { (Get-Date $_-12-25).DayOfWeek -eq "Sunday" }

### Find Christmas holiday for any day and/or year[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=101 "Edit section: Find Christmas holiday for any day and/or year")]

   
function Get-ChristmasHoliday  
{  
    [CmdletBinding()]  
    [OutputType([PSCustomObject])]  
    Param  
    (  
        [Parameter(Mandatory=$false,  
                   ValueFromPipeline=$true,  
                   ValueFromPipelineByPropertyName=$true,  
                   Position=0)]  
        [ValidateRange(1,9999)]  
        [int[]]  
        $Year = (Get-Date).Year  
    )  
   
    Process  
    {  
        [datetime]$christmas = Get-Date $Year/12/25  
   
        switch ($christmas.DayOfWeek)  
        {  
            "Sunday"   {[datetime[]]$dates = 1..5 | ForEach-Object {$christmas.AddDays($_)}}  
            "Monday"   {[datetime[]]$dates = $christmas, $christmas.AddDays(1)}  
            "Saturday" {[datetime[]]$dates = $christmas.AddDays(-2), $christmas.AddDays(-1)}  
            Default    {[datetime[]]$dates = $christmas.AddDays(-1), $christmas}  
        }  
   
        $dates | Group-Object  -Property Year |  
                 Select-Object -Property @{Name="Year"     ; Expression={$_.Name}},  
                                         @{Name="DayOfWeek"; Expression={$christmas.DayOfWeek}},  
                                         @{Name="Christmas"; Expression={$christmas.ToString("MM/dd/yyyy")}},  
                                         @{Name="DaysOff"  ; Expression={$_.Group | ForEach-Object {$_.ToString("MM/dd/yyyy")}}}  
    }  
}  
 

Satisfy the task requirement:

   
2008..2121 | Get-ChristmasHoliday | where DayOfWeek -match Su  
 

Output:

Year DayOfWeek Christmas  DaysOff                                            
---- --------- ---------  -------                                            
2011    Sunday 12/25/2011 {12/26/2011, 12/27/2011, 12/28/2011, 12/29/2011...}
2016    Sunday 12/25/2016 {12/26/2016, 12/27/2016, 12/28/2016, 12/29/2016...}
2022    Sunday 12/25/2022 {12/26/2022, 12/27/2022, 12/28/2022, 12/29/2022...}
2033    Sunday 12/25/2033 {12/26/2033, 12/27/2033, 12/28/2033, 12/29/2033...}
2039    Sunday 12/25/2039 {12/26/2039, 12/27/2039, 12/28/2039, 12/29/2039...}
2044    Sunday 12/25/2044 {12/26/2044, 12/27/2044, 12/28/2044, 12/29/2044...}
2050    Sunday 12/25/2050 {12/26/2050, 12/27/2050, 12/28/2050, 12/29/2050...}
2061    Sunday 12/25/2061 {12/26/2061, 12/27/2061, 12/28/2061, 12/29/2061...}
2067    Sunday 12/25/2067 {12/26/2067, 12/27/2067, 12/28/2067, 12/29/2067...}
2072    Sunday 12/25/2072 {12/26/2072, 12/27/2072, 12/28/2072, 12/29/2072...}
2078    Sunday 12/25/2078 {12/26/2078, 12/27/2078, 12/28/2078, 12/29/2078...}
2089    Sunday 12/25/2089 {12/26/2089, 12/27/2089, 12/28/2089, 12/29/2089...}
2095    Sunday 12/25/2095 {12/26/2095, 12/27/2095, 12/28/2095, 12/29/2095...}
2101    Sunday 12/25/2101 {12/26/2101, 12/27/2101, 12/28/2101, 12/29/2101...}
2107    Sunday 12/25/2107 {12/26/2107, 12/27/2107, 12/28/2107, 12/29/2107...}
2112    Sunday 12/25/2112 {12/26/2112, 12/27/2112, 12/28/2112, 12/29/2112...}
2118    Sunday 12/25/2118 {12/26/2118, 12/27/2118, 12/28/2118, 12/29/2118...}

Get days off for a random year:

   
Get-ChristmasHoliday -Year (2008..2121 | Get-Random)  
 

Output:

Year DayOfWeek Christmas  DaysOff                 
---- --------- ---------  -------                 
2110  Thursday 12/25/2110 {12/24/2110, 12/25/2110}

Get days off for the current year using the  **Year**  property returned by  `Get-Date`:

   
(Get-Date | Get-ChristmasHoliday).DaysOff  
 

Output:

12/26/2016
12/27/2016
12/28/2016
12/29/2016
12/30/2016

Get days off for the current year as  `[DateTime]`  objects:

   
(Get-Date | Get-ChristmasHoliday).DaysOff | Get-Date  
 

Output:

Monday, December 26, 2016 12:00:00 AM
Tuesday, December 27, 2016 12:00:00 AM
Wednesday, December 28, 2016 12:00:00 AM
Thursday, December 29, 2016 12:00:00 AM
Friday, December 30, 2016 12:00:00 AM

## [Prolog](https://rosettacode.org/wiki/Category:Prolog "Category:Prolog")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=102 "Edit section: Prolog")]

Works with SWI-Prolog;

main() :-  
    christmas_days_falling_on_sunday(2011, 2121, SundayList),  
    writeln(SundayList).  
   
christmas_days_falling_on_sunday(StartYear, EndYear, SundayList) :-  
    numlist(StartYear, EndYear, YearRangeList),  
    [include](http://pauillac.inria.fr/~deransar/prolog/bips.html)(is_christmas_day_a_sunday, YearRangeList, SundayList).  
   
is_christmas_day_a_sunday(Year) :-  
    Date = date(Year, 12, 25),  
    day_of_the_week(Date, DayOfTheWeek),  
    DayOfTheWeek == 7.  
 

Output:

?- main.
[2011,2016,2022,2033,2039,2044,2050,2061,2067,2072,2078,2089,2095,2101,2107,2112,2118]
true.

## [PureBasic](https://rosettacode.org/wiki/Category:PureBasic "Category:PureBasic")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=103 "Edit section: PureBasic")]

PureBasic's internal Date() is limited between 1970-01-01 00:00:00 and 2038-01-19 03:14:07

For i=2008 To 2037  
  If DayOfWeek(Date(i,12,25,0,0,0))=0  
    PrintN(Str(i))  
  EndIf  
Next

## [Python](https://rosettacode.org/wiki/Category:Python "Category:Python")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=104 "Edit section: Python")]

```python
from calendar import weekday, SUNDAY  
   
[year for year in range(2008, 2122) if weekday(year, 12, 25) == SUNDAY]
```  

Output:

[2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

The function  `calendar.weekday`  accepts all dates between 1/1/1 and 9999/12/31, and uses the  [proleptic Gregorian calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar)  before adoption of the  [Gregorian calendar](https://en.wikipedia.org/wiki/Gregorian_calendar)  in 1582. There is no gap between 1582/10/4 and 1582/10/15, as can be seen with  `print(calendar.calendar(1582))`.

  
Or, in terms of datetime:

**Works with**:  [Python](https://rosettacode.org/wiki/Python "Python")  version 3.7

'''Days of the week'''  
   
from datetime import date  
from itertools import islice  
   
   
# xmasIsSunday :: Int -> Bool  
def xmasIsSunday(y):  
    '''True if Dec 25 in the given year is a Sunday.'''  
    return 6 == date(y, 12, 25).weekday()  
   
   
# main :: IO ()  
def main():  
    '''Years between 2008 and 2121 with 25 Dec on a Sunday'''  
   
    xs = list(filter(  
        xmasIsSunday,  
        enumFromTo(2008)(2121)  
    ))  
    total = len(xs)  
    print(  
        fTable(main.__doc__ + ':\n\n' + '(Total ' + str(total) + ')\n')(  
            lambda i: str(1 + i)  
        )(str)(index(xs))(  
            enumFromTo(0)(total - 1)  
        )  
    )  
   
   
# GENERIC -------------------------------------------------  
   
# enumFromTo :: (Int, Int) -> [Int]  
def enumFromTo(m):  
    '''Integer enumeration from m to n.'''  
    return lambda n: list(range(m, 1 + n))  
   
   
# index (!!) :: [a] -> Int -> a  
def index(xs):  
    '''Item at given (zero-based) index.'''  
    return lambda n: None if 0 > n else (  
        xs[n] if (  
            hasattr(xs, "__getitem__")  
        ) else next(islice(xs, n, None))  
    )  
   
   
# unlines :: [String] -> String  
def unlines(xs):  
    '''A single string formed by the intercalation  
       of a list of strings with the newline character.  
    '''  
    return '\n'.join(xs)  
   
   
#  FORMATTING ---------------------------------------------  
# fTable :: String -> (a -> String) ->  
#                     (b -> String) -> (a -> b) -> [a] -> String  
def fTable(s):  
    '''Heading -> x display function -> fx display function ->  
                     f -> xs -> tabular string.  
    '''  
    def go(xShow, fxShow, f, xs):  
        ys = [xShow(x) for x in xs]  
        w = max(map(len, ys))  
        return s + '\n' + '\n'.join(map(  
            lambda x, y: y.rjust(w, ' ') + ' -> ' + fxShow(f(x)),  
            xs, ys  
        ))  
    return lambda xShow: lambda fxShow: lambda f: lambda xs: go(  
        xShow, fxShow, f, xs  
    )  
   
   
# MAIN --  
if __name__ == '__main__':  
    main()

Output:

Years between 2008 and 2121 with 25 Dec on a Sunday:

(Total 17)

 1 -> 2011
 2 -> 2016
 3 -> 2022
 4 -> 2033
 5 -> 2039
 6 -> 2044
 7 -> 2050
 8 -> 2061
 9 -> 2067
10 -> 2072
11 -> 2078
12 -> 2089
13 -> 2095
14 -> 2101
15 -> 2107
16 -> 2112
17 -> 2118

## [R](https://rosettacode.org/wiki/Category:R "Category:R")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=105 "Edit section: R")]

years <- 2008:2121  
xmas <- as.POSIXlt(paste0(years, '/12/25'))  
years[xmas$wday==0]  
# 2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118  
   
# Also:  
xmas=seq(as.Date("2008/12/25"), as.Date("2121/12/25"), by="year")  
as.numeric(format(xmas[weekdays(xmas)== 'Sunday'], "%Y"))  
   
# Still another solution, using ISOdate and weekdays  
with(list(years=2008:2121), years[weekdays(ISOdate(years, 12, 25)) == "Sunday"])  
   
# Or with "subset"  
subset(data.frame(years=2008:2121), weekdays(ISOdate(years, 12, 25)) == "Sunday")$years  
   
# Simply replace "Sunday" with whatever it's named in your country,  
# or set locale first, with  
Sys.setlocale(cat="LC_ALL", "en")  
   
# Under MS Windows, write instead  
Sys.setlocale("LC_ALL", "English")

## [Racket](https://rosettacode.org/wiki/Category:Racket "Category:Racket")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=106 "Edit section: Racket")]

   
#lang racket  
   
(require racket/date)  
   
(define (xmas-on-sunday? year)  
  (zero? (date-week-day (seconds->date (find-seconds 0 0 12 25 12 year)))))  
   
(for ([y (in-range 2008 2121)] #:when (xmas-on-sunday? y))  
  (displayln y))  
 

## [REBOL](https://rosettacode.org/wiki/Category:REBOL "Category:REBOL")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=107 "Edit section: REBOL")]

rebol [  
	Title: "Yuletide Holiday"  
	URL: http://rosettacode.org/wiki/Yuletide_Holiday  
]  
   
for y 2008 2121 1 [  
	d: to-date reduce [y 12 25]  
	if 7 = d/weekday [prin [y ""]]  
]

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Red](https://rosettacode.org/wiki/Category:Red "Category:Red")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=108 "Edit section: Red")]

Red []  
repeat yy 114 [  
  d: to-date reduce [25 12 (2007 + yy )]  
  if 7 = d/weekday [ print d ] ;; 7 = sunday  
]  
;; or  
print "version 2"  
   
d: to-date [25 12 2008]  
while [d <= 25/12/2121 ] [  
  if 7 = d/weekday [   
    print rejoin [d/day '. d/month '. d/year ]   
  ]   
  d/year: d/year + 1  
]  
 

Output:

25-Dec-2011

25-Dec-2016
25-Dec-2022
25-Dec-2033
25-Dec-2039
25-Dec-2044
25-Dec-2050
25-Dec-2061
25-Dec-2067
25-Dec-2072
25-Dec-2078
25-Dec-2089
25-Dec-2095
25-Dec-2101
25-Dec-2107
25-Dec-2112
25-Dec-2118
version 2
25.12.2011
25.12.2016
25.12.2022
25.12.2033
25.12.2039
25.12.2044
25.12.2050
25.12.2061
25.12.2067
25.12.2072
25.12.2078
25.12.2089
25.12.2095
25.12.2101
25.12.2107
25.12.2112
25.12.2118
>> 

## [REXX](https://rosettacode.org/wiki/Category:REXX "Category:REXX")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=109 "Edit section: REXX")]

### using DATE weekday[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=110 "Edit section: using DATE weekday")]

The extended DATE parameters (arguments 2 and 3) are only supported by the newer REXX interpreters.

    do year=2008 to 2121  
    if date('w', year'1225', 's') == 'Sunday' then say year  
    end

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

### using DATE base[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=111 "Edit section: using DATE base")]

The extended DATE parameters (arguments 2 and 3) are only supported by the newer REXX interpreters.

    do year=2008 to 2121  
    if date('b', year'1225', 's') // 7 == 6 then say year  
    end

**output**  is the same as above

### using DATE iso[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=112 "Edit section: using DATE iso")]

Works with Regina REXX only. Works with ooRexx

The extended DATE parameters (arguments 2 and 3) are only supported by the newer REXX interpreters.

Programming note: The **ISO**  option of the **date**  BIF is a Regina extension.

Language note: the DATE built-in function always returns the day-of-week in English, no matter what the native language is in effect.

/*REXX program displays in which  years  12/25  (December 25th)   falls on a  Sunday.   */  
parse arg start finish .                         /*get the  START  and  FINISH  years.  */  
if  start=='' |  start==","  then  start=2008    /*Not specified?  Then use the default.*/  
if finish=='' | finish==","  then finish=2121    /* "       "        "   "   "     "    */  
   
      do y=start  to finish                      /*process all the years specified.     */  
   
      if date('Weekday', y"-12-25", 'ISO')\=='Sunday'  then iterate  
   
   /* if date('w'      , y"-12-25", 'i'  ) ···       (same as above).  */  
   /*          ↑↑↑↑↑↑   ↑↑↑↑↑↑↑↑↑↑  ↑↑↑                                */  
   /*          option   yyyy-mm-dd  fmt                                */  
   
      say 'December 25th,'    y    "falls on a Sunday."  
      end  /*y*/  
                                                 /*stick a fork in it,  we're all done. */

**output**  when using the default input:

December 25th, 2011 falls on a Sunday.
December 25th, 2016 falls on a Sunday.
December 25th, 2022 falls on a Sunday.
December 25th, 2033 falls on a Sunday.
December 25th, 2039 falls on a Sunday.
December 25th, 2044 falls on a Sunday.
December 25th, 2050 falls on a Sunday.
December 25th, 2061 falls on a Sunday.
December 25th, 2067 falls on a Sunday.
December 25th, 2072 falls on a Sunday.
December 25th, 2078 falls on a Sunday.
December 25th, 2089 falls on a Sunday.
December 25th, 2095 falls on a Sunday.
December 25th, 2101 falls on a Sunday.
December 25th, 2107 falls on a Sunday.
December 25th, 2112 falls on a Sunday.
December 25th, 2118 falls on a Sunday.

### old school DOW[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=113 "Edit section: old school DOW")]

This DOW (day-of-week) version will work with any version of a REXX interpreter.

/*REXX program (old school) displays in which years 12/25 (Dec. 25th) falls on a Sunday.*/  
parse arg start finish .                         /*get the  START  and  FINISH  years.  */  
if  start=='' |  start==","  then  start=2008    /*Not specified?  Then use the default.*/  
if finish=='' | finish==","  then finish=2121    /* "       "        "   "   "     "    */  
   
      do y=start  to finish                      /*process all the years specified.     */  
      if dow(12,25,y)==1  then say 'December 25th,'       y       "falls on a Sunday."  
      end   /*y*/  
exit                                             /*stick a fork in it,  we're all done. */  
/*──────────────────────────────────────────────────────────────────────────────────────*/  
dow: procedure; parse arg m,d,y;                    if m<3  then do;  m=m+12;  y=y-1;  end  
     yL=left(y,2);  yr=right(y,2);  w=(d + (m+1)*26%10+yr+yr%4+yL%4+5*yL) // 7  
     if w==0  then w=7;   return w               /*Sunday=1,  Monday=2,  ···  Saturday=7*/

**output**  when using the default input:

December 25th, 2011 falls on a Sunday.
December 25th, 2016 falls on a Sunday.
December 25th, 2022 falls on a Sunday.
December 25th, 2033 falls on a Sunday.
December 25th, 2039 falls on a Sunday.
December 25th, 2044 falls on a Sunday.
December 25th, 2050 falls on a Sunday.
December 25th, 2061 falls on a Sunday.
December 25th, 2067 falls on a Sunday.
December 25th, 2072 falls on a Sunday.
December 25th, 2078 falls on a Sunday.
December 25th, 2089 falls on a Sunday.
December 25th, 2095 falls on a Sunday.
December 25th, 2101 falls on a Sunday.
December 25th, 2107 falls on a Sunday.
December 25th, 2112 falls on a Sunday.
December 25th, 2118 falls on a Sunday.

## [Ring](https://rosettacode.org/wiki/Category:Ring "Category:Ring")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=114 "Edit section: Ring")]

   
for n = 2008 to 2121  
    if n < 2100 leap = n - 1900 else leap = n - 1904 ok  
    m = (((n-1900)%7) + floor(leap/4) + 27) % 7   
    if m = 4 see "25 Dec " + n + nl ok  
next  
 

## [Ruby](https://rosettacode.org/wiki/Category:Ruby "Category:Ruby")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=115 "Edit section: Ruby")]

require 'date'  
   
(2008..2121).each {|year| puts "25 Dec #{year}" if Date.new(year, 12, 25).sunday? }

Output:

25 Dec 2011
25 Dec 2016
25 Dec 2022
25 Dec 2033
25 Dec 2039
25 Dec 2044
25 Dec 2050
25 Dec 2061
25 Dec 2067
25 Dec 2072
25 Dec 2078
25 Dec 2089
25 Dec 2095
25 Dec 2101
25 Dec 2107
25 Dec 2112
25 Dec 2118

Or using the Time class

(2008..2121).each {|year| puts "25 Dec #{year}" if Time.local(year, 12, 25).sunday?}

Output:

25 Dec 2011
25 Dec 2016
25 Dec 2022
25 Dec 2033
25 Dec 2039
25 Dec 2044
25 Dec 2050
25 Dec 2061
25 Dec 2067
25 Dec 2072
25 Dec 2078
25 Dec 2089
25 Dec 2095
25 Dec 2101
25 Dec 2107
25 Dec 2112
25 Dec 2118

(Note: The Time class could not handle dates beyond 2038 prior to Ruby 1.9.2.[[1]](https://www.ruby-lang.org/en/news/2010/08/18/ruby-1-9.2-released/))

## [Run BASIC](https://rosettacode.org/wiki/Category:Run_BASIC "Category:Run BASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=116 "Edit section: Run BASIC")]

for year = 2008 to 2121  
 if val(date$("12-25-";year)) mod 7 = 5 then print "For ";year;"xmas is Sunday"  
next year

For 2011 xmas is Sunday
For 2016 xmas is Sunday
For 2022 xmas is Sunday
For 2033 xmas is Sunday
For 2039 xmas is Sunday
For 2044 xmas is Sunday
For 2050 xmas is Sunday
For 2061 xmas is Sunday
For 2067 xmas is Sunday
For 2072 xmas is Sunday
For 2078 xmas is Sunday
For 2089 xmas is Sunday
For 2095 xmas is Sunday
For 2101 xmas is Sunday
For 2107 xmas is Sunday
For 2112 xmas is Sunday
For 2118 xmas is Sunday

## [Rust](https://rosettacode.org/wiki/Category:Rust "Category:Rust")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=117 "Edit section: Rust")]

```rust  
extern crate chrono;  
   
use chrono::prelude::*;  
   
fn main() {  
    let years = (2008..2121).filter(|&y| Local.ymd(y, 12, 25).weekday() == Weekday::Sun).collect::<Vec<i32>>();  
    println!("Years = {:?}", years);  
}
```  
Output:

Years = [2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118]

## [SAS](https://rosettacode.org/wiki/Category:SAS "Category:SAS")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=118 "Edit section: SAS")]

data _null_;  
do y=2008 to 2121;  
a=mdy(12,25,y);  
if weekday(a)=1 then put y;  
end;  
run;  
   
/* 2011 2016 2022 2033 2039 2044 2050 2061 2067  
   2072 2078 2089 2095 2101 2107 2112 2118 */

## [S-BASIC](https://rosettacode.org/mw/index.php?title=Category:S-BASIC&action=edit&redlink=1 "Category:S-BASIC (page does not exist)")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=119 "Edit section: S-BASIC")]

   
$constant SUNDAY = 0  
   
rem - compute p mod q  
function mod(p, q = integer) = integer  
end = p - q * (p/q)  
   
comment  
    return day of week (Sun = 0, Mon = 1, etc.) for a  
    given Gregorian calendar date using Zeller's congruence  
end  
function dayofweek (mo, da, yr = integer) = integer  
    var y, c, z = integer  
    if mo < 3 then  
        begin  
            mo = mo + 10  
            yr = yr - 1  
        end  
    else mo = mo - 2  
    y = mod(yr,100)  
    c = int(yr / 100)  
    z = int((26 * mo - 2) / 10)  
    z = z + da + y + int(y/4) + int(c/4) - 2 * c + 777  
    z = mod(z,7)  
end = z  
   
rem - main program  
var year = integer  
print "Christmas will fall on a Sunday in"  
for year=2008 to 2121  
   if dayofweek(12,25,year) = SUNDAY then  
      print year  
next year  
end  
 

Output:

Christmas will fall on a Sunday in
 2011
 2016
 2011
 2033
 2039
 2044
 2050
 2061
 2067
 2072
 2078
 2089
 2095
 2101
 2107
 2112
 2118

## [Scala](https://rosettacode.org/wiki/Category:Scala "Category:Scala")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=120 "Edit section: Scala")]

**Library:**  [Scala](https://rosettacode.org/wiki/Category:Scala "Category:Scala")

### JDK (discouraged)[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=121 "Edit section: JDK (discouraged)")]

[import](https://scala-lang.org/) java.util.{ Calendar, GregorianCalendar }  
[import](https://scala-lang.org/) Calendar.{ DAY_OF_WEEK, DECEMBER, SUNDAY }  
   
[object](https://scala-lang.org/) DayOfTheWeek [extends](https://scala-lang.org/) App {  
  [val](https://scala-lang.org/) years = 2008 to 2121  
   
  [val](https://scala-lang.org/) yuletide =  
    years.filter(year => ([new](https://scala-lang.org/) GregorianCalendar(year, DECEMBER, 25)).get(DAY_OF_WEEK) == SUNDAY)  
   
  // If you want a test: (optional)  
  assert(yuletide ==  
    Seq(2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061,  
      2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118))  
   
  println(yuletide.mkString(  
    s"${yuletide.length} Years between ${years.head} and ${years.last}" +  
      " including where Christmas is observed on Sunday:\n", ", ", "."))  
}

### JDK >= 8 (recommended)[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=122 "Edit section: JDK >= 8 (recommended)")]

#### Naive programming[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=123 "Edit section: Naive programming")]

[import](https://scala-lang.org/) java.time.{ DayOfWeek, LocalDate }  
   
[object](https://scala-lang.org/) DayOfTheWeek1 [extends](https://scala-lang.org/) App {  
  [val](https://scala-lang.org/) years = 2008 to 2121  
  [val](https://scala-lang.org/) yuletide = [for](https://scala-lang.org/) {  
    year <- years  
    [if](https://scala-lang.org/) LocalDate.of(year, 12, 25).getDayOfWeek() == DayOfWeek.SUNDAY  
  } [yield](https://scala-lang.org/) year  
   
  println(yuletide.mkString(  
    s"${yuletide.count(p => true)} Years between ${years.head} and ${years.last}" +  
      " including where Christmas is observed on Sunday:\n", ", ", "."))  
}

#### Idiomatic programming[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=124 "Edit section: Idiomatic programming")]

[import](https://scala-lang.org/) java.time.{ DayOfWeek, LocalDate }  
   
[object](https://scala-lang.org/) DayOfTheWeek1 [extends](https://scala-lang.org/) App {  
  [val](https://scala-lang.org/) years = 2008 to 2121  
  [val](https://scala-lang.org/) yuletide =  
    years.filter(year => (LocalDate.of(year, 12, 25).getDayOfWeek() == DayOfWeek.SUNDAY))  
   
  // If you want a test: (optional)  
  assert(yuletide ==  
    Seq(2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061,  
      2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118))  
   
  println(yuletide.mkString(  
    s"${yuletide.length} Years between ${years.head} and ${years.last}" +  
      " including where Christmas is observed on Sunday:\n", ", ", "."))  
}

#### Tail recursion[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=125 "Edit section: Tail recursion")]

[import](https://scala-lang.org/) java.time.{ DayOfWeek, LocalDate }  
[import](https://scala-lang.org/) scala.annotation.tailrec  
   
[object](https://scala-lang.org/) DayOfTheWeek3 [extends](https://scala-lang.org/) App {  
  [val](https://scala-lang.org/) years = 2008 to 2121  
  [val](https://scala-lang.org/) yuletide = {  
    @tailrec  
    [def](https://scala-lang.org/) inner(anni: List[Int], accu: List[Int]): List[Int] = {  
      [if](https://scala-lang.org/) (anni == Nil) accu  
      [else](https://scala-lang.org/) inner(anni.tail, accu ++  
        ([if](https://scala-lang.org/) (LocalDate.of(anni.head, 12, 25).getDayOfWeek() == DayOfWeek.SUNDAY) List(anni.head)  
        [else](https://scala-lang.org/) Nil))  
    }  
    inner(years.toList, Nil)  
  }  
   
  // If you want a test: (optional)  
  assert(yuletide ==  
    Seq(2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061,  
      2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118))  
   
  println(yuletide.mkString(  
    s"${yuletide.length} Years between ${years.head} and ${years.last}" +  
      " including where Christmas is observed on Sunday:\n", ", ", "."))  
}

Output of all solutions:

Years between 2008 and 2121 including when Christmas is observed on Sunday:
2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118.

## [Scheme](https://rosettacode.org/wiki/Category:Scheme "Category:Scheme")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=126 "Edit section: Scheme")]

(define (day-of-week year month day)  
(if (< month 3)  
    (begin (set! month (+ month 12)) (set! year (- year 1))))  
(+ 1  
   (remainder (+ 5 day (quotient (* (+ 1 month) 13) 5)  
                 year (quotient year 4) (* (quotient year 100) 6) (quotient year 400))  
              7)))  
   
(define (task)  
(let loop ((y 2121) (v '()))  
(if (< y 2008)  
    v  
    (loop (- y 1)  
          (if (= 7 (day-of-week y 12 25))  
              (cons y v)  
              v)))))  
   
(task)  
; (2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118)

## [Seed7](https://rosettacode.org/wiki/Category:Seed7 "Category:Seed7")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=127 "Edit section: Seed7")]

The library  [time.s7i](http://seed7.sourceforge.net/libraries/time.htm)  defines the function  [dayOfWeek](http://seed7.sourceforge.net/libraries/time.htm#dayOfWeek%28in_time%29), which returns 1 for monday, 2 for tuesday, and so on up to 7 for sunday.

$ include "seed7_05.s7i";  
  include "time.s7i";  
   
const proc: main is func  
  local  
    var integer: year is 0;  
  begin  
    for year range 2008 to 2122 do  
      if dayOfWeek(date(year, 12, 25)) = 7 then  
        writeln("Christmas comes on a sunday in " <& year);     
      end if;  
    end for;  
  end func;

Output:

Christmas comes on a sunday in 2011
Christmas comes on a sunday in 2016
Christmas comes on a sunday in 2022
Christmas comes on a sunday in 2033
Christmas comes on a sunday in 2039
Christmas comes on a sunday in 2044
Christmas comes on a sunday in 2050
Christmas comes on a sunday in 2061
Christmas comes on a sunday in 2067
Christmas comes on a sunday in 2072
Christmas comes on a sunday in 2078
Christmas comes on a sunday in 2089
Christmas comes on a sunday in 2095
Christmas comes on a sunday in 2101
Christmas comes on a sunday in 2107
Christmas comes on a sunday in 2112
Christmas comes on a sunday in 2118

## [Sidef](https://rosettacode.org/wiki/Category:Sidef "Category:Sidef")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=128 "Edit section: Sidef")]

**Translation of**:  [Perl](https://rosettacode.org/wiki/Day_of_the_week#Perl)

require('Time::Local')  
   
for year in (2008 .. 2121) {  
    var time = %S<Time::Local>.timelocal(0,0,0,25,11,year)  
    var wd = Time(time).local.wday  
    if (wd == 0) {  
        say "25 Dec #{year} is Sunday"  
    }  
}

Output:

25 Dec 2011 is Sunday
25 Dec 2016 is Sunday
25 Dec 2022 is Sunday
25 Dec 2033 is Sunday
25 Dec 2039 is Sunday
25 Dec 2044 is Sunday
25 Dec 2050 is Sunday
25 Dec 2061 is Sunday
25 Dec 2067 is Sunday
25 Dec 2072 is Sunday
25 Dec 2078 is Sunday
25 Dec 2089 is Sunday
25 Dec 2095 is Sunday
25 Dec 2101 is Sunday
25 Dec 2107 is Sunday
25 Dec 2112 is Sunday
25 Dec 2118 is Sunday

## [Smalltalk](https://rosettacode.org/wiki/Category:Smalltalk "Category:Smalltalk")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=129 "Edit section: Smalltalk")]

2008 to: 2121 do: [ :year | |date|  
     date := Date newDay: 25 monthIndex: 12 year: year.  
     date dayName = #Sunday  
       ifTrue: [ date displayNl ]  
]

Output:

25-Dec-2011
25-Dec-2016
25-Dec-2022
25-Dec-2033
25-Dec-2039
25-Dec-2044
25-Dec-2050
25-Dec-2061
25-Dec-2067
25-Dec-2072
25-Dec-2078
25-Dec-2089
25-Dec-2095
25-Dec-2101
25-Dec-2107
25-Dec-2112
25-Dec-2118

## [SQL](https://rosettacode.org/wiki/Category:SQL "Category:SQL")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=130 "Edit section: SQL")]

SQL has good support for date functions; care must be taken with NLS settings (globalization support), in the code below the date format language is passed in as an argument to the relevant function.

SELECT EXTRACT(YEAR FROM dt) AS year_with_xmas_on_sunday  
FROM   (   
         SELECT  add_months(DATE '2008-12-25', 12 * (level - 1)) AS dt  
         FROM    dual  
         CONNECT BY level <= 2121 - 2008 + 1  
       )  
   
WHERE  to_char(dt, 'Dy', 'nls_date_language=English') = 'Sun'  
ORDER  BY 1  
;

  

Output:

YEAR_WITH_XMAS_ON_SUNDAY
------------------------
                    2011
                    2016
                    2022
                    2033
                    2039
                    2044
                    2050
                    2061
                    2067
                    2072
                    2078
                    2089
                    2095
                    2101
                    2107
                    2112
                    2118

17 rows selected.

## [Stata](https://rosettacode.org/wiki/Category:Stata "Category:Stata")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=131 "Edit section: Stata")]

clear  
sca n=2121-2008+1  
set obs `=n'  
gen year=2007+_n  
list if dow(mdy(12,25,year))==0, noobs sep(0)  
   
  +------+  
  | year |  
  |------|  
  | 2011 |  
  | 2016 |  
  | 2022 |  
  | 2033 |  
  | 2039 |  
  | 2044 |  
  | 2050 |  
  | 2061 |  
  | 2067 |  
  | 2072 |  
  | 2078 |  
  | 2089 |  
  | 2095 |  
  | 2101 |  
  | 2107 |  
  | 2112 |  
  | 2118 |  
  +------+

### Mata[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=132 "Edit section: Mata")]

year=2008::2121  
select(year,dow(mdy(12,25,year)):==0)

## [Suneido](https://rosettacode.org/wiki/Category:Suneido "Category:Suneido")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=133 "Edit section: Suneido")]

year = 2008  
while (year <= 2121)  
    {  
    if Date('#' $ year $ '1225').WeekDay() is 0  
        Print(year)  
    ++year  
    }

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Standard ML](https://rosettacode.org/wiki/Category:Standard_ML "Category:Standard ML")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=134 "Edit section: Standard ML")]

(* Call:  yearsOfSundayXmas(2008, 2121)   *)  
fun yearsOfSundayXmas(fromYear, toYear) =  
  if fromYear>toYear then  
    ()  
  else  
    let  
      val d = Date.date {year=fromYear, month=Date.Dec, day=25,   
              hour=0, minute=0, second=0,  
                      offset=SOME Time.zeroTime}  
      val wd = Date.weekDay d  
    in  
      if wd=Date.Sun then  
        (  
          print(Int.toString fromYear ^ "\n");  
          yearsOfSundayXmas(fromYear+1, toYear)  
        )  
      else  
        yearsOfSundayXmas(fromYear+1, toYear)  
    end;

Output:

- yearsOfSundayXmas(2008, 2121);
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

  

## [Swift](https://rosettacode.org/wiki/Category:Swift "Category:Swift")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=135 "Edit section: Swift")]

import Cocoa  
   
var year=2008  
let formatter=NSDateFormatter()  
formatter.dateFormat = "yyyy-MM-dd"  
   
let gregorian:NSCalendar! = NSCalendar(calendarIdentifier: NSCalendarIdentifierGregorian)  
while (year<2122){  
    var date:NSDate!=formatter.dateFromString(String(year)+"-12-25")  
    var components=gregorian.components(NSCalendarUnit.CalendarUnitWeekday, fromDate: date)  
    var dayOfWeek:NSInteger=components.weekday  
    if(dayOfWeek==1){  
        println(year)  
    }  
    year++  
}

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Tcl](https://rosettacode.org/wiki/Category:Tcl "Category:Tcl")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=136 "Edit section: Tcl")]

**Works with**:  [Tcl](https://rosettacode.org/wiki/Tcl "Tcl")  version 8.5

package require Tcl 8.5  
   
for {set y 2008} {$y <= 2121} {incr y} {  
    if {[clock format [clock scan "$y-12-25" -format {%Y-%m-%d}] -format %w] == 0} {  
        puts "xmas $y is a sunday"  
    }  
}

Output:

xmas 2011 is a sunday
xmas 2016 is a sunday
xmas 2022 is a sunday
xmas 2033 is a sunday
xmas 2039 is a sunday
xmas 2044 is a sunday
xmas 2050 is a sunday
xmas 2061 is a sunday
xmas 2067 is a sunday
xmas 2072 is a sunday
xmas 2078 is a sunday
xmas 2089 is a sunday
xmas 2095 is a sunday
xmas 2101 is a sunday
xmas 2107 is a sunday
xmas 2112 is a sunday
xmas 2118 is a sunday

## [TI-83 BASIC](https://rosettacode.org/wiki/Category:TI-83_BASIC "Category:TI-83 BASIC")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=137 "Edit section: TI-83 BASIC")]

**Works with**  TI-84+/SE only

   
:For(A,2008,2121  
:If dayofWk(A,12,25)=1  
:Disp A  
:End  
 

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118
Done

## [TUSCRIPT](https://rosettacode.org/wiki/Category:TUSCRIPT "Category:TUSCRIPT")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=138 "Edit section: TUSCRIPT")]

   
$$ MODE TUSCRIPT  
PRINT "25th of December will be a Sunday in the following years: "  
LOOP year=2008,2121  
SET dayofweek = DATE (number,25,12,year,nummer)  
IF (dayofweek==7) PRINT year  
ENDLOOP  
 

Output:

25th of December will be a Sunday in the following years:
2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [UNIX Shell](https://rosettacode.org/wiki/Category:UNIX_Shell "Category:UNIX Shell")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=139 "Edit section: UNIX Shell")]

Unix commands may use  _time_t_  to count seconds since the  [epoch](https://rosettacode.org/wiki/Show_the_epoch "Show the epoch"). For systems with 32-bit time, the counter overflows during 19 January 2038. These scripts continue to 2121 and may need a system with 64-bit time, to prevent the overflow.

### With GNU date[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=140 "Edit section: With GNU date")]

This solution uses date -d, which seems to be a  [GNU](https://rosettacode.org/wiki/GNU "GNU")  extension, so it only works with those systems.

**Works with**:  [bash](https://rosettacode.org/wiki/Bash "Bash")

#! /bin/bash  
   
for (( i=2008; i<=2121; ++i ))  
do  
 date -d "$i-12-25"  
done  |grep Sun  
   
exit 0

The first lines of output (from a GNU/Linux system with 32bit time_t, date version 6.9) are

Sun Dec 25 00:00:00 CET 2011  
Sun Dec 25 00:00:00 CET 2016  
Sun Dec 25 00:00:00 CET 2022  
Sun Dec 25 00:00:00 CET 2033  
date: invalid date `2038-12-25'

I.e., starting from year 2038, the  date  command (which uses the glibc library, at least on GNU systems), is not able to recognise the date as a valid one!

_Different machine/OS version (64 bit time_t):_  This is the same command run on RedHat Linux.

bash-3.00$ date --version  
date (coreutils) 5.2.1  
Written by David MacKenzie.  
   
Copyright (C) 2004 Free Software Foundation, Inc.  
This is free software; see the source for copying conditions.  There is NO  
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  
bash-3.00$ uname -a  
Linux brslln01 2.6.9-67.ELsmp #1 SMP Wed Nov 7 13:56:44 EST 2007 x86_64 x86_64 x86_64 GNU/Linux  
bash-3.00$ for((i=2009; i <= 2121; i++)); do  date -d "$i-12-25" |egrep Sun; done  
Sun Dec 25 00:00:00 GMT 2011  
Sun Dec 25 00:00:00 GMT 2016  
Sun Dec 25 00:00:00 GMT 2022  
Sun Dec 25 00:00:00 GMT 2033  
Sun Dec 25 00:00:00 GMT 2039  
Sun Dec 25 00:00:00 GMT 2044  
Sun Dec 25 00:00:00 GMT 2050  
Sun Dec 25 00:00:00 GMT 2061  
Sun Dec 25 00:00:00 GMT 2067  
Sun Dec 25 00:00:00 GMT 2072  
Sun Dec 25 00:00:00 GMT 2078  
Sun Dec 25 00:00:00 GMT 2089  
Sun Dec 25 00:00:00 GMT 2095  
Sun Dec 25 00:00:00 GMT 2101  
Sun Dec 25 00:00:00 GMT 2107  
Sun Dec 25 00:00:00 GMT 2112  
Sun Dec 25 00:00:00 GMT 2118  
bash-3.00$

### With GNU date and GNU seq ([UnixPipes](https://rosettacode.org/wiki/Category:UnixPipes "Category:UnixPipes"))[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=141 "Edit section: With GNU date and GNU seq (UnixPipes)")]

Like the previous solution, this solution uses date -d, which seems to be a  [GNU](https://rosettacode.org/wiki/GNU "GNU")  extension. Output is same as previous solution.

seq 2008 2121 | xargs -IYEAR -n 1 date +%c -d 'Dec 25 YEAR' | grep Sun

### With Unix cal[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=142 "Edit section: With Unix cal")]

The  `cal`  command is a tradition since Version 6 AT&T UNIX. This solution assumes that  `cal`  will always output a calendar in this format.

$ cal 12 2011 
   December 2011
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
                    

This format always puts Sunday in columns 1 and 2. The solution uses  _tail_  to delete the first 2 lines (month, year, names of days),  _cut_  to extract Sunday's columns, and  _grep_  to check if "25" appears in those columns.

**Works with**:  [Bourne Shell](https://rosettacode.org/wiki/Bourne_Shell "Bourne Shell")

y=2008  
while test $y -lt 2122; do  
	cal 12 $y | tail +3 | cut -c1-2 | grep -Fq 25 && echo 25 Dec $y  
	y=`expr $y + 1`  
done

Running this script with  [OpenBSD](https://rosettacode.org/wiki/OpenBSD "OpenBSD"), the output is identical to the C# program. OpenBSD  _cal_  accepts any year from 1 to 9999, so 2008 to 2122 is well within range.

### With zsh[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=143 "Edit section: With zsh")]

zmodload zsh/datetime  
for (( year = 2010; year <= 2121; year++ ));  
  if [[ $(strftime '%A' $(strftime -r '%F' $year-12-25)) == Sunday ]] print $year

If the system has 32-bit time, this script will malfunction for years >= 2038; it will print no year from 2038 to 2121 (unless today is Sunday, then it prints every year from 2038 to 2121). This happens because  _strftime -r '%F' $year-12-25_  yields -1 for an out-of-range date, and  _strftime '%A' -1_  yields name of today.

## [Ursala](https://rosettacode.org/wiki/Category:Ursala "Category:Ursala")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=144 "Edit section: Ursala")]

A standard library,  `stt`, provides basic date manipulation functions, and is imported in this example. Unix era times denominated in seconds since 1969 (excluding leap seconds) are represented as natural numbers with unlimited precision. Results are valid for the arbitrarily distant future assuming the Gregorian calendar remains in effect.

The algorithm relies on the  `string_to_time`  function converting a date expressed as a character string to seconds without needing a weekday field in the input, and the  `time_to_string`  function outputting the corresponding date with the weekday included. The output is then filtered for Sundays.

#import std  
#import nat  
#import stt  
   
christmases = time_to_string* string_to_time*TS 'Dec 25 0:0:0 '-*@hS %nP* nrange/2008 2121  
   
#show+  
   
sunday_years = ~&zS sep` * =]'Sun'*~ christmases

Output:

2011                            
2016                            
2022                            
2033                            
2039                            
2044                            
2050                            
2061                            
2067                            
2072                            
2078                            
2089                            
2095                            
2101                            
2107                            
2112
2118

## [VBA](https://rosettacode.org/wiki/Category:VBA "Category:VBA")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=145 "Edit section: VBA")]

Option Explicit  
   
Sub MainDayOfTheWeek()  
    Debug.Print "Xmas will be a Sunday in : " & XmasSunday(2008, 2121)  
End Sub  
   
Private Function XmasSunday(firstYear As Integer, lastYear As Integer) As String  
Dim i As Integer, temp$  
    For i = firstYear To lastYear  
        If Weekday(CDate("25/12/" & i)) = vbSunday Then temp = temp & ", " & i  
    Next  
    XmasSunday = Mid(temp, 2)  
End Function

Output:

Xmas will be a Sunday in :  2011, 2016, 2022, 2033, 2039, 2044, 2050, 2061, 2067, 2072, 2078, 2089, 2095, 2101, 2107, 2112, 2118

## [VBScript](https://rosettacode.org/wiki/Category:VBScript "Category:VBScript")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=146 "Edit section: VBScript")]

For year = 2008 To 2121  
    If Weekday(DateSerial(year, 12, 25)) = 1 Then  
        WScript.Echo year  
    End If  
Next

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Vedit macro language](https://rosettacode.org/wiki/Category:Vedit_macro_language "Category:Vedit macro language")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=147 "Edit section: Vedit macro language")]

Buf_Switch(Buf_Free)  
for (#3 = 2008; #3 < 2122; #3++) {  
    Reg_Set(10, "12/25/")  
    Num_Str(#3, 10, LEFT+APPEND)  
    if (JDate(@10) % 7 == 0) {  
	Num_Ins(#3, NOCR)  
    }  
}

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Visual Objects](https://rosettacode.org/wiki/Category:Visual_Objects "Category:Visual Objects")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=148 "Edit section: Visual Objects")]

   
local i as dword  
   
for i := 2008 upto 2121    
	if DOW(ConDate(i, 12, 25)) = 1     
		? AsString(i)  
	endif                
next i   
 

Output:

2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118

## [Wortel](https://rosettacode.org/wiki/Category:Wortel "Category:Wortel")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=149 "Edit section: Wortel")]

!-&y = 0 `.getDay. @new Date[y 11 25] @range[2008 2121]

Returns:

[2011 2016 2022 2033 2039 2044 2050 2061 2067 2072 2078 2089 2095 2101 2107 2112 2118]

## [XPL0](https://rosettacode.org/wiki/Category:XPL0 "Category:XPL0")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=150 "Edit section: XPL0")]

The original routine in the library only worked correctly between the years 1980 and 2099. It was upgraded with this new routine that handles all dates in the Gregorian calendar, from 1583 onward. It's based on Zeller's Congruence.

include c:\cxpl\codes;                  \intrinsic 'code' declarations  
   
func    WeekDay(Year, Month, Day);      \Return day of week (0=Sat 1=Sun..6=Fri)  
int     Year, Month, Day;  
[if Month<=2 then [Month:= Month+12;  Year:= Year-1];  
return rem((Day + (Month+1)*26/10 + Year + Year/4 + Year/100*6 + Year/400) / 7);  
];      \WeekDay  
   
   
int     Year;  
[for Year:= 2008 to 2121 do  
    if WeekDay(Year, 12, 25) = 1 then   \25th of December is a Sunday  
        [IntOut(0, Year);  CrLf(0)];  
]

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [Yabasic](https://rosettacode.org/wiki/Category:Yabasic "Category:Yabasic")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=151 "Edit section: Yabasic")]

**Translation of**:  [FreeBASIC](https://rosettacode.org/wiki/Day_of_the_week#FreeBASIC)

sub wd(m, d, y)  
  If m < 3 Then        // If m = 1 Or m = 2 Then  
    m = m + 12  
    y = y - 1  
  End If  
  Return mod((y + int(y / 4) - int(y / 100) + int(y / 400) + d + int((153 * m + 8) / 5)), 7)  
End sub  
   
// ------=< MAIN >=------  
   
For yr = 2008 To 2121  
  If wd(12, 25, yr) = 0 Then  
    Print "Dec 25 ", yr  
  EndIf  
Next

## [zkl](https://rosettacode.org/wiki/Category:Zkl "Category:Zkl")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=152 "Edit section: zkl")]

ISO dates, monday is 1, sunday is 7

var [const] D=Time.Date;  
foreach y in ([2008..2121]){  
   if (D.Sunday==D.weekDay(y,12,25)) println(y)  
}

Or, in a more functional manner:

var [const] D=Time.Date;  
[2008..2121].filter(fcn(y){ D.Sunday==D.weekDay(y,12,25) }).println()

Output:

2011
2016
2022
2033
2039
2044
2050
2061
2067
2072
2078
2089
2095
2101
2107
2112
2118

## [zonnon](https://rosettacode.org/wiki/Category:Zonnon "Category:Zonnon")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=153 "Edit section: zonnon")]

   
module Main;  
(*Access to Mono System package *)  
import System;  
   
var  
	now: System.DateTime;  
begin  
	now := System.DateTime.Now;  
	System.Console.Write(now.ToString("yyyy-MM-dd :"));  
	System.Console.WriteLine(now.DayOfWeek);  
end Main.  
 

Output:

2017-12-05 :Tuesday

## [ZX Spectrum Basic](https://rosettacode.org/wiki/Category:ZX_Spectrum_Basic "Category:ZX Spectrum Basic")[[edit](https://rosettacode.org/mw/index.php?title=Day_of_the_week&action=edit&section=154 "Edit section: ZX Spectrum Basic")]

**Translation of**:  [BASIC](https://rosettacode.org/wiki/Day_of_the_week#BASIC)

10 CLS   
20 FOR y=2008 TO 2121  
30 LET year=y: LET m=12: LET d=25: GO SUB 1000  
40 IF wd=0 THEN PRINT d;" ";m;" ";y  
50 NEXT y  
60 STOP   
1000 REM week day  
1010 IF m=1 OR m=2 THEN LET m=m+12: LET year=year-1  
1020 LET wd=FN m(year+INT (year/4)-INT (year/100)+INT (year/400)+d+INT ((153*m+8)/5),7)  
1030 RETURN   
1100 DEF FN m(a,b)=a-INT (a/b)*b
