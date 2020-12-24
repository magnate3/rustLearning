
# Zero to the zero power

[![Task](http://rosettacode.org/mw/images/thumb/b/ba/Rcode-button-task-crushed.png/64px-Rcode-button-task-crushed.png)](http://rosettacode.org/wiki/Category:Solutions_by_Programming_Task "Category:Solutions by Programming Task")

**Zero to the zero power**  
You are encouraged to  [solve this task](http://rosettacode.org/wiki/Rosetta_Code:Solve_a_Task "Rosetta Code:Solve a Task")  according to the task description, using any language you may know.

Some computer programming languages are not exactly consistent (with other computer programming languages)  
when _raising zero to the zeroth power_: **00**

  

Task

Show the results of raising zero to the zeroth power.

  
If your computer language objects to **0**0** or **0^0** at compile time, you may also try something like:
```
           x = 0
           y = 0
           z = x**y
           say  'z='  z
```
  
**Show the result here.**  
And of course use any symbols or notation that is supported in your computer programming language for exponentiation.

  

See also

-   The Wiki entry:  [Zero to the power of zero](https://en.wikipedia.org/wiki/Exponentiation#Zero_to_the_power_of_zero "wp:Exponentiation").
-   The Wiki entry:  [History of differing points of view](https://en.wikipedia.org/wiki/Exponentiation#History_of_differing_points_of_view "wp:Exponentiation").
-   The MathWorld™ entry:  [exponent laws](http://mathworld.wolfram.com/ExponentLaws.html).
    -   Also, in the above MathWorld™ entry, see formula (**9**):  {\displaystyle x^{0}=1}.
-   The OEIS entry:  [The special case of zero to the zeroth power](https://oeis.org/wiki/The_special_case_of_zero_to_the_zeroth_power)

  
  

## Contents

[[hide](http://rosettacode.org/wiki/Zero_to_the_zero_power#)]

-   [1  8th](http://rosettacode.org/wiki/Zero_to_the_zero_power#8th)
-   [2  ARM Assembly](http://rosettacode.org/wiki/Zero_to_the_zero_power#ARM_Assembly)
-   [3  AutoHotkey](http://rosettacode.org/wiki/Zero_to_the_zero_power#AutoHotkey)
-   [4  Ada](http://rosettacode.org/wiki/Zero_to_the_zero_power#Ada)
-   [5  ALGOL 68](http://rosettacode.org/wiki/Zero_to_the_zero_power#ALGOL_68)
-   [6  APL](http://rosettacode.org/wiki/Zero_to_the_zero_power#APL)
-   [7  Applesoft BASIC](http://rosettacode.org/wiki/Zero_to_the_zero_power#Applesoft_BASIC)
-   [8  AWK](http://rosettacode.org/wiki/Zero_to_the_zero_power#AWK)
-   [9  BaCon](http://rosettacode.org/wiki/Zero_to_the_zero_power#BaCon)
-   [10  Bc](http://rosettacode.org/wiki/Zero_to_the_zero_power#Bc)
-   [11  Befunge](http://rosettacode.org/wiki/Zero_to_the_zero_power#Befunge)
-   [12  Bracmat](http://rosettacode.org/wiki/Zero_to_the_zero_power#Bracmat)
-   [13  Burlesque](http://rosettacode.org/wiki/Zero_to_the_zero_power#Burlesque)
-   [14  BBC BASIC](http://rosettacode.org/wiki/Zero_to_the_zero_power#BBC_BASIC)
-   [15  C](http://rosettacode.org/wiki/Zero_to_the_zero_power#C)
-   [16  C++](http://rosettacode.org/wiki/Zero_to_the_zero_power#C.2B.2B)
-   [17  C#](http://rosettacode.org/wiki/Zero_to_the_zero_power#C.23)
-   [18  Caché ObjectScript](http://rosettacode.org/wiki/Zero_to_the_zero_power#Cach.C3.A9_ObjectScript)
-   [19  Clojure](http://rosettacode.org/wiki/Zero_to_the_zero_power#Clojure)
-   [20  COBOL](http://rosettacode.org/wiki/Zero_to_the_zero_power#COBOL)
-   [21  ColdFusion](http://rosettacode.org/wiki/Zero_to_the_zero_power#ColdFusion)
    -   [21.1  Classic tag based CFML](http://rosettacode.org/wiki/Zero_to_the_zero_power#Classic_tag_based_CFML)
    -   [21.2  Script Based CFML](http://rosettacode.org/wiki/Zero_to_the_zero_power#Script_Based_CFML)
-   [22  Common Lisp](http://rosettacode.org/wiki/Zero_to_the_zero_power#Common_Lisp)
-   [23  D](http://rosettacode.org/wiki/Zero_to_the_zero_power#D)
-   [24  Dc](http://rosettacode.org/wiki/Zero_to_the_zero_power#Dc)
-   [25  EchoLisp](http://rosettacode.org/wiki/Zero_to_the_zero_power#EchoLisp)
-   [26  Eiffel](http://rosettacode.org/wiki/Zero_to_the_zero_power#Eiffel)
-   [27  Elena](http://rosettacode.org/wiki/Zero_to_the_zero_power#Elena)
-   [28  Elixir](http://rosettacode.org/wiki/Zero_to_the_zero_power#Elixir)
-   [29  ERRE](http://rosettacode.org/wiki/Zero_to_the_zero_power#ERRE)
-   [30  F#](http://rosettacode.org/wiki/Zero_to_the_zero_power#F.23)
-   [31  Factor](http://rosettacode.org/wiki/Zero_to_the_zero_power#Factor)
-   [32  Falcon](http://rosettacode.org/wiki/Zero_to_the_zero_power#Falcon)
-   [33  Forth](http://rosettacode.org/wiki/Zero_to_the_zero_power#Forth)
-   [34  Fortran](http://rosettacode.org/wiki/Zero_to_the_zero_power#Fortran)
-   [35  FreeBASIC](http://rosettacode.org/wiki/Zero_to_the_zero_power#FreeBASIC)
-   [36  Gambas](http://rosettacode.org/wiki/Zero_to_the_zero_power#Gambas)
-   [37  Go](http://rosettacode.org/wiki/Zero_to_the_zero_power#Go)
-   [38  FutureBasic](http://rosettacode.org/wiki/Zero_to_the_zero_power#FutureBasic)
-   [39  Groovy](http://rosettacode.org/wiki/Zero_to_the_zero_power#Groovy)
-   [40  Haskell](http://rosettacode.org/wiki/Zero_to_the_zero_power#Haskell)
-   [41  HolyC](http://rosettacode.org/wiki/Zero_to_the_zero_power#HolyC)
-   [42  Icon and Unicon](http://rosettacode.org/wiki/Zero_to_the_zero_power#Icon_and_Unicon)
-   [43  J](http://rosettacode.org/wiki/Zero_to_the_zero_power#J)
-   [44  Java](http://rosettacode.org/wiki/Zero_to_the_zero_power#Java)
-   [45  JavaScript](http://rosettacode.org/wiki/Zero_to_the_zero_power#JavaScript)
    -   [45.1  Math.pow](http://rosettacode.org/wiki/Zero_to_the_zero_power#Math.pow)
    -   [45.2  exponentiation operator (**)](http://rosettacode.org/wiki/Zero_to_the_zero_power#exponentiation_operator_.28.2A.2A.29)
-   [46  jq](http://rosettacode.org/wiki/Zero_to_the_zero_power#jq)
-   [47  Jsish](http://rosettacode.org/wiki/Zero_to_the_zero_power#Jsish)
-   [48  Julia](http://rosettacode.org/wiki/Zero_to_the_zero_power#Julia)
-   [49  K](http://rosettacode.org/wiki/Zero_to_the_zero_power#K)
-   [50  Kotlin](http://rosettacode.org/wiki/Zero_to_the_zero_power#Kotlin)
-   [51  Lua](http://rosettacode.org/wiki/Zero_to_the_zero_power#Lua)
-   [52  M2000 Interpreter](http://rosettacode.org/wiki/Zero_to_the_zero_power#M2000_Interpreter)
-   [53  Maple](http://rosettacode.org/wiki/Zero_to_the_zero_power#Maple)
-   [54  Mathematica](http://rosettacode.org/wiki/Zero_to_the_zero_power#Mathematica)
-   [55  MATLAB / Octave](http://rosettacode.org/wiki/Zero_to_the_zero_power#MATLAB_.2F_Octave)
-   [56  Mercury](http://rosettacode.org/wiki/Zero_to_the_zero_power#Mercury)
-   [57  Microsoft Small Basic](http://rosettacode.org/wiki/Zero_to_the_zero_power#Microsoft_Small_Basic)
-   [58  min](http://rosettacode.org/wiki/Zero_to_the_zero_power#min)
-   [59  MiniScript](http://rosettacode.org/wiki/Zero_to_the_zero_power#MiniScript)
-   [60  МК-61/52](http://rosettacode.org/wiki/Zero_to_the_zero_power#.D0.9C.D0.9A-61.2F52)
-   [61  Neko](http://rosettacode.org/wiki/Zero_to_the_zero_power#Neko)
-   [62  NetRexx](http://rosettacode.org/wiki/Zero_to_the_zero_power#NetRexx)
-   [63  NewLISP](http://rosettacode.org/wiki/Zero_to_the_zero_power#NewLISP)
-   [64  Nial](http://rosettacode.org/wiki/Zero_to_the_zero_power#Nial)
-   [65  Nim](http://rosettacode.org/wiki/Zero_to_the_zero_power#Nim)
-   [66  OCaml](http://rosettacode.org/wiki/Zero_to_the_zero_power#OCaml)
-   [67  Oforth](http://rosettacode.org/wiki/Zero_to_the_zero_power#Oforth)
-   [68  Ol](http://rosettacode.org/wiki/Zero_to_the_zero_power#Ol)
-   [69  ooRexx](http://rosettacode.org/wiki/Zero_to_the_zero_power#ooRexx)
-   [70  PARI/GP](http://rosettacode.org/wiki/Zero_to_the_zero_power#PARI.2FGP)
-   [71  Pascal](http://rosettacode.org/wiki/Zero_to_the_zero_power#Pascal)
-   [72  Perl](http://rosettacode.org/wiki/Zero_to_the_zero_power#Perl)
-   [73  Perl 6](http://rosettacode.org/wiki/Zero_to_the_zero_power#Perl_6)
-   [74  Phix](http://rosettacode.org/wiki/Zero_to_the_zero_power#Phix)
-   [75  PHP](http://rosettacode.org/wiki/Zero_to_the_zero_power#PHP)
-   [76  PicoLisp](http://rosettacode.org/wiki/Zero_to_the_zero_power#PicoLisp)
-   [77  PL/I](http://rosettacode.org/wiki/Zero_to_the_zero_power#PL.2FI)
-   [78  PowerShell](http://rosettacode.org/wiki/Zero_to_the_zero_power#PowerShell)
-   [79  PureBasic](http://rosettacode.org/wiki/Zero_to_the_zero_power#PureBasic)
-   [80  Pyret](http://rosettacode.org/wiki/Zero_to_the_zero_power#Pyret)
-   [81  Python](http://rosettacode.org/wiki/Zero_to_the_zero_power#Python)
    -   [81.1  Python3](http://rosettacode.org/wiki/Zero_to_the_zero_power#Python3)
    -   [81.2  Python2](http://rosettacode.org/wiki/Zero_to_the_zero_power#Python2)
-   [82  R](http://rosettacode.org/wiki/Zero_to_the_zero_power#R)
-   [83  Racket](http://rosettacode.org/wiki/Zero_to_the_zero_power#Racket)
-   [84  REXX](http://rosettacode.org/wiki/Zero_to_the_zero_power#REXX)
-   [85  Ring](http://rosettacode.org/wiki/Zero_to_the_zero_power#Ring)
-   [86  Ruby](http://rosettacode.org/wiki/Zero_to_the_zero_power#Ruby)
-   [87  Rust](http://rosettacode.org/wiki/Zero_to_the_zero_power#Rust)
-   [88  S-lang](http://rosettacode.org/wiki/Zero_to_the_zero_power#S-lang)
-   [89  Scala](http://rosettacode.org/wiki/Zero_to_the_zero_power#Scala)
-   [90  Scheme](http://rosettacode.org/wiki/Zero_to_the_zero_power#Scheme)
-   [91  Seed7](http://rosettacode.org/wiki/Zero_to_the_zero_power#Seed7)
-   [92  Sidef](http://rosettacode.org/wiki/Zero_to_the_zero_power#Sidef)
-   [93  Sinclair ZX81 BASIC](http://rosettacode.org/wiki/Zero_to_the_zero_power#Sinclair_ZX81_BASIC)
-   [94  Smalltalk](http://rosettacode.org/wiki/Zero_to_the_zero_power#Smalltalk)
-   [95  smart BASIC](http://rosettacode.org/wiki/Zero_to_the_zero_power#smart_BASIC)
-   [96  SQL](http://rosettacode.org/wiki/Zero_to_the_zero_power#SQL)
-   [97  Standard ML](http://rosettacode.org/wiki/Zero_to_the_zero_power#Standard_ML)
-   [98  Stata](http://rosettacode.org/wiki/Zero_to_the_zero_power#Stata)
-   [99  Swift](http://rosettacode.org/wiki/Zero_to_the_zero_power#Swift)
-   [100  Tcl](http://rosettacode.org/wiki/Zero_to_the_zero_power#Tcl)
-   [101  TI-83_BASIC](http://rosettacode.org/wiki/Zero_to_the_zero_power#TI-83_BASIC)
-   [102  uBasic/4tH](http://rosettacode.org/wiki/Zero_to_the_zero_power#uBasic.2F4tH)
-   [103  Ursa](http://rosettacode.org/wiki/Zero_to_the_zero_power#Ursa)
-   [104  VBA](http://rosettacode.org/wiki/Zero_to_the_zero_power#VBA)
-   [105  VBScript](http://rosettacode.org/wiki/Zero_to_the_zero_power#VBScript)
-   [106  Visual Basic .NET](http://rosettacode.org/wiki/Zero_to_the_zero_power#Visual_Basic_.NET)
-   [107  XLISP](http://rosettacode.org/wiki/Zero_to_the_zero_power#XLISP)
-   [108  zkl](http://rosettacode.org/wiki/Zero_to_the_zero_power#zkl)

## [8th](http://rosettacode.org/wiki/Category:8th "Category:8th")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=1 "Edit section: 8th")]

   
0 0 ^ .  
 

Output:

1

## [ARM Assembly](http://rosettacode.org/wiki/Category:ARM_Assembly "Category:ARM Assembly")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=2 "Edit section: ARM Assembly")]

## [AutoHotkey](http://rosettacode.org/wiki/Category:AutoHotkey "Category:AutoHotkey")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=3 "Edit section: AutoHotkey")]

[MsgBox](http://www.autohotkey.com/docs/commands/MsgBox.htm) % 0 ** 0

Output:

1

## [Ada](http://rosettacode.org/wiki/Category:Ada "Category:Ada")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=4 "Edit section: Ada")]

with Ada.Text_IO, Ada.Integer_Text_IO, Ada.Long_Integer_Text_IO,  
  Ada.Long_Long_Integer_Text_IO, Ada.Float_Text_IO, Ada.Long_Float_Text_IO,  
  Ada.Long_Long_Float_Text_IO;  
use  Ada.Text_IO, Ada.Integer_Text_IO, Ada.Long_Integer_Text_IO,  
  Ada.Long_Long_Integer_Text_IO, Ada.Float_Text_IO, Ada.Long_Float_Text_IO,  
  Ada.Long_Long_Float_Text_IO;  
   
procedure Test5 is  
   
   I    : Integer           := 0;  
   LI   : Long_Integer      := 0;  
   LLI  : Long_Long_Integer := 0;  
   F    : Float             := 0.0;  
   LF   : Long_Float        := 0.0;  
   LLF  : Long_Long_Float   := 0.0;  
   Zero : Natural           := 0;  
   
begin  
   Put ("Integer           0^0 = ");   
   Put (I ** Zero, 2);   New_Line;  
   Put ("Long Integer      0^0 = ");  
   Put (LI ** Zero, 2);  New_Line;  
   Put ("Long Long Integer 0^0 = ");  
   Put (LLI ** Zero, 2); New_Line;  
   Put ("Float           0.0^0 = ");             
   Put (F ** Zero);   New_Line;  
   Put ("Long Float      0.0^0 = ");        
   Put (LF ** Zero);  New_Line;  
   Put ("Long Long Float 0.0^0 = ");   
   Put (LLF ** Zero); New_Line;  
end Test5;  
 

Output:

Integer           0^0 =  1
Long Integer      0^0 =  1
Long Long Integer 0^0 =  1
Float           0.0^0 =  1.00000E+00
Long Float      0.0^0 =  1.00000000000000E+00
Long Long Float 0.0^0 =  1.00000000000000000E+00

## [ALGOL 68](http://rosettacode.org/wiki/Category:ALGOL_68 "Category:ALGOL 68")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=5 "Edit section: ALGOL 68")]

**Works with**:  [ALGOL 68G](http://rosettacode.org/wiki/ALGOL_68G "ALGOL 68G")  version Any - tested with release 2.6.win32

print( ( 0 ^ 0, newline ) )  
 

Output:

         +1

## [APL](http://rosettacode.org/wiki/Category:APL "Category:APL")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=6 "Edit section: APL")]

      0*0  
1

## [Applesoft BASIC](http://rosettacode.org/wiki/Category:Applesoft_BASIC "Category:Applesoft BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=7 "Edit section: Applesoft BASIC")]

]? 0^0
1

## [AWK](http://rosettacode.org/wiki/Category:AWK "Category:AWK")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=8 "Edit section: AWK")]

   
# syntax: GAWK -f ZERO_TO_THE_ZERO_POWER.AWK  
BEGIN {  
    print(0 ^ 0)  
    exit(0)  
}  
 

Output:

1

## [BaCon](http://rosettacode.org/wiki/Category:BaCon "Category:BaCon")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=9 "Edit section: BaCon")]

PRINT POW(0, 0)

Output:

prompt$ ./zerotothezero
1

## [Bc](http://rosettacode.org/wiki/Category:Bc "Category:Bc")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=10 "Edit section: Bc")]

   
0 ^ 0  
 

Output:

1

## [Befunge](http://rosettacode.org/wiki/Category:Befunge "Category:Befunge")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=11 "Edit section: Befunge")]

**Befunge-93**  doesn't have explicit support for exponentiation, but there are a couple of fingerprint extensions for  **Befunge-98**  which add that functionality. The example below makes use of the  **FPDP**  fingerprint (double precision floating point).

Note that the result is potentially dependent on the underlying language of the interpreter, but all those tested so far have returned 1. Interpreters that don't support  **Befunge-98**, or don't support this fingerprint, should just terminate (possibly with a warning).

"PDPF"4#@(0F0FYP)@

Output:

1.000000

## [Bracmat](http://rosettacode.org/wiki/Category:Bracmat "Category:Bracmat")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=12 "Edit section: Bracmat")]

0^0

Output:

1

## [Burlesque](http://rosettacode.org/wiki/Category:Burlesque "Category:Burlesque")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=13 "Edit section: Burlesque")]

   
blsq ) 0.0 0.0?^  
1.0  
blsq ) 0 0?^  
1  
 

## [BBC BASIC](http://rosettacode.org/wiki/Category:BBC_BASIC "Category:BBC BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=14 "Edit section: BBC BASIC")]

      PRINT 0^0

Output:

1

## [C](http://rosettacode.org/wiki/Category:C "Category:C")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=15 "Edit section: C")]

**Works with**:  [C99](http://rosettacode.org/wiki/C99 "C99")

This example uses the standard  `pow`  function in the math library. 0^0 is given as 1.

#include <stdio.h>  
#include <math.h>  
#include <complex.h>  
   
int main()  
{  
	[printf](https://www.opengroup.org/onlinepubs/009695399/functions/printf.html)("0 ^ 0 = %f\n", [pow](https://www.opengroup.org/onlinepubs/009695399/functions/pow.html)(0,0));  
        double complex c = [cpow](https://www.opengroup.org/onlinepubs/009695399/functions/cpow.html)(0,0);  
	[printf](https://www.opengroup.org/onlinepubs/009695399/functions/printf.html)("0+0i ^ 0+0i = %f+%fi\n", [creal](https://www.opengroup.org/onlinepubs/009695399/functions/creal.html)(c), [cimag](https://www.opengroup.org/onlinepubs/009695399/functions/cimag.html)(c));  
	return 0;  
}

Output:

0 ^ 0 = 1.000000
0+0i ^ 0+0i = nan+nani

## [C++](http://rosettacode.org/wiki/Category:C%2B%2B "Category:C++")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=16 "Edit section: C++")]

#include <iostream>  
#include <cmath>  
#include <complex>  
   
int main()  
{  
  std::cout << "0 ^ 0 = " << std::pow(0,0) << std::endl;  
  std::cout << "0+0i ^ 0+0i = " <<  
    std::pow(std::complex<double>(0),std::complex<double>(0)) << std::endl;  
  return 0;  
}

Output:

0 ^ 0 = 1
0+0i ^ 0+0i = (nan,nan)

## [C#](http://rosettacode.org/wiki/Category:C_sharp "Category:C sharp")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=17 "Edit section: C#")]

using System;  
   
namespace ZeroToTheZeroeth  
{  
    class Program  
    {  
        static void Main(string[] args)  
        {  
            double k = Math.Pow(0, 0);  
            Console.Write("0^0 is {0}", k);             
        }  
    }  
}

Output:

0^0 is 1

## [Caché ObjectScript](http://rosettacode.org/wiki/Category:Cach%C3%A9_ObjectScript "Category:Caché ObjectScript")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=18 "Edit section: Caché ObjectScript")]

ZEROPOW  
  // default behavior is incorrect:  
  set (x,y) = 0  
  w !,"0 to the 0th power (wrong): "_(x**y)  ; will output 0  
   
  // if one or both of the values is a double, this works  
  set (x,y) = $DOUBLE(0)  
  w !,"0 to the 0th power (right): "_(x**y)  
   
  quit

Output:

SAMPLES>do ^ZEROPOW

0 to the 0th power (wrong): 0

0 to the 0th power (right): 1   

## [Clojure](http://rosettacode.org/wiki/Category:Clojure "Category:Clojure")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=19 "Edit section: Clojure")]

user=> (use 'clojure.math.numeric-tower)
user=> (expt 0 0)
1

; alternative java-interop route:
user=> (Math/pow 0 0)
1.0

## [COBOL](http://rosettacode.org/wiki/Category:COBOL "Category:COBOL")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=20 "Edit section: COBOL")]

identification division.  
program-id. zero-power-zero-program.  
data division.  
working-storage section.  
77  n                         pic 9.  
procedure division.  
    compute n = 0**0.  
    display n upon console.  
    stop run.

Output:

1

## [ColdFusion](http://rosettacode.org/wiki/Category:ColdFusion "Category:ColdFusion")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=21 "Edit section: ColdFusion")]

### Classic tag based CFML[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=22 "Edit section: Classic tag based CFML")]

   
<cfset zeroPowerTag = 0^0>  
<cfoutput>"#zeroPowerTag#"</cfoutput>  
 

Output:

"1"

### Script Based CFML[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=23 "Edit section: Script Based CFML")]

<cfscript>  
  zeroPower = 0^0;  
  writeOutput( zeroPower );  
</cfscript>

Output:

1

## [Common Lisp](http://rosettacode.org/wiki/Category:Common_Lisp "Category:Common Lisp")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=24 "Edit section: Common Lisp")]

> (expt 0 0)
1

## [D](http://rosettacode.org/wiki/Category:D "Category:D")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=25 "Edit section: D")]

void main() {  
    import std.stdio, std.math, std.bigint, std.complex;  
   
    writeln("Int:     ", 0 ^^ 0);  
    writeln("Ulong:   ", 0UL ^^ 0UL);  
    writeln("Float:   ", 0.0f ^^ 0.0f);  
    writeln("Double:  ", 0.0 ^^ 0.0);  
    writeln("Real:    ", 0.0L ^^ 0.0L);  
    writeln("pow:     ", pow(0, 0));  
    writeln("BigInt:  ", 0.BigInt ^^ 0);  
    writeln("Complex: ", complex(0.0, 0.0) ^^ 0);  
}

Output:

Int:     1
Ulong:   1
Float:   1
Double:  1
Real:    1
pow:     1
BigInt:  1
Complex: 1+0i

## [Dc](http://rosettacode.org/wiki/Category:Dc "Category:Dc")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=26 "Edit section: Dc")]

0 0^p  
 

Output:

1

## [EchoLisp](http://rosettacode.org/wiki/Category:EchoLisp "Category:EchoLisp")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=27 "Edit section: EchoLisp")]

   
;; trying the 16 combinations  
;; all return the integer 1  
   
(lib 'bigint)  
(define zeroes '(integer: 0 inexact=float: 0.000 complex: 0+0i bignum: #0))  
(for* ((z1 zeroes) (z2 zeroes)) (write (expt z1 z2)))  
    →  1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1   
 

## [Eiffel](http://rosettacode.org/wiki/Category:Eiffel "Category:Eiffel")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=28 "Edit section: Eiffel")]

print (0^0)

Output:

1

## [Elena](http://rosettacode.org/wiki/Category:Elena "Category:Elena")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=29 "Edit section: Elena")]

ELENA 4.x

import extensions;  
   
public program()  
{  
    console.printLine("0^0 is ",0.power:0)  
}

Output:

0^0 is 0

## [Elixir](http://rosettacode.org/wiki/Category:Elixir "Category:Elixir")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=30 "Edit section: Elixir")]

Elixir uses Erlang's  `:math`  for power operations and can handle zero to the zero power.

   
:math.pow(0,0)  
 

Output:

1.0

## [ERRE](http://rosettacode.org/wiki/Category:ERRE "Category:ERRE")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=31 "Edit section: ERRE")]

   
.....  
PRINT(0^0)  
.....  
 

Output:

 1

## [F#](http://rosettacode.org/wiki/Category:F_Sharp "Category:F Sharp")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=32 "Edit section: F#")]

In the REPL:

> let z = 0.**0.;;

val z : float = 1.0

## [Factor](http://rosettacode.org/wiki/Category:Factor "Category:Factor")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=33 "Edit section: Factor")]

USING: math.functions.private ; ! ^complex  
0 0 ^  
C{ 0 0 } C{ 0 0 } ^complex

Output:

--- Data stack:
NAN: 8000000000000
C{ NAN: 8000000000000 NAN: 8000000000000 }

  

## [Falcon](http://rosettacode.org/wiki/Category:Falcon "Category:Falcon")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=34 "Edit section: Falcon")]

**VBA/Python programmer's approach not sure if it's the most falconic way**

   
/* created by Aykayayciti Earl Lamont Montgomery  
April 9th, 2018 */  
   
x = 0  
y = 0  
z = x**y  
> "z=", z  
   
 

Output:

z=1
[Finished in 0.2s]

## [Forth](http://rosettacode.org/wiki/Category:Forth "Category:Forth")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=35 "Edit section: Forth")]

0e 0e f** f.

Output:

1.

Of course in an embedded program we would be tempted to "pre-calculate" the answer :-)

: ^0     DROP  1 ;

Output:

0 ^0 . 1 ok

## [Fortran](http://rosettacode.org/wiki/Category:Fortran "Category:Fortran")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=36 "Edit section: Fortran")]

   
program zero  
double precision :: i, j  
double complex :: z1, z2  
i = 0.0D0  
j = 0.0D0  
z1 = (0.0D0,0.0D0)  
z2 = (0.0D0,0.0D0)  
write(*,*) 'When integers are used, we have 0^0 = ', 0**0  
write(*,*) 'When double precision numbers are used, we have 0.0^0.0 = ', i**j  
write(*,*) 'When complex numbers are used, we have (0.0+0.0i)^(0.0+0.0i) = ', z1**z2  
end program  
 

Output:

 When integers are used, we have 0^0 =            1
 When double precision numbers are used, we have 0.0^0.0 =    1.0000000000000000     
 When complex numbers are used, we have (0.0+0.0i)^(0.0+0.0i) =  (             NaN,             NaN)

## [FreeBASIC](http://rosettacode.org/wiki/Category:FreeBASIC "Category:FreeBASIC")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=37 "Edit section: FreeBASIC")]

' FB 1.05.0 Win64  
   
Print "0 ^ 0 ="; 0 ^ 0  
Sleep

Output:

0 ^ 0 = 1

## [Gambas](http://rosettacode.org/wiki/Category:Gambas "Category:Gambas")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=38 "Edit section: Gambas")]

**[Click this link to run this code](https://gambas-playground.proko.eu/?gist=7d505dbe89227e9b4423f92ef12d6829)**

[Public](http://gambasdoc.org/help/lang/public) [Sub](http://gambasdoc.org/help/lang/sub) Main()  
   
[Print](http://gambasdoc.org/help/lang/print) 0 ^ 0  
   
[End](http://gambasdoc.org/help/lang/end)

Output:

1

## [Go](http://rosettacode.org/wiki/Category:Go "Category:Go")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=39 "Edit section: Go")]

Go does not have an exponentiation operator but has functions in the standard library for three types, float64, complex128, and big.Int. As of Go 1.3, all are documented to return 1.

package main  
   
import (  
    "fmt"  
    "math"  
    "math/big"  
    "math/cmplx"  
)  
   
func main() {  
    fmt.Println("float64:    ", math.Pow(0, 0))  
    var b [big.Int](https://golang.org/search?q=big.Int)  
    fmt.Println("big integer:", b.Exp(&b, &b, nil))  
    fmt.Println("complex:    ", cmplx.Pow(0, 0))  
}

Output:

float64:     1
big integer: 1
complex:     (1+0i)

## [FutureBasic](http://rosettacode.org/wiki/Category:FutureBasic "Category:FutureBasic")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=40 "Edit section: FutureBasic")]

   
include "ConsoleWindow"  
   
print 0^0  
 

Output:

1

## [Groovy](http://rosettacode.org/wiki/Category:Groovy "Category:Groovy")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=41 "Edit section: Groovy")]

**Translation of**:  [Java](http://rosettacode.org/wiki/Zero_to_the_zero_power#Java)

Test:

[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) 0**0

Output:

1

## [Haskell](http://rosettacode.org/wiki/Category:Haskell "Category:Haskell")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=42 "Edit section: Haskell")]

import Data.Complex  
   
main = do  
  [print](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:print) $ 0 ^ 0  
  [print](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:print) $ 0.0 ^ 0  
  [print](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:print) $ 0 ^^ 0  
  [print](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:print) $ 0 ** 0  
  [print](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:print) $ (0 :+ 0) ^ 0  
  [print](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:print) $ (0 :+ 0) ** (0 :+ 0)

Output:

1
1.0
1.0
1.0
1.0 :+ 0.0
NaN :+ NaN

## [HolyC](http://rosettacode.org/wiki/Category:HolyC "Category:HolyC")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=43 "Edit section: HolyC")]

F64 a = 0 ` 0;  
Print("0 ` 0 = %5.3f\n", a);

Output:

0 ` 0 = 1.000

## [Icon](http://rosettacode.org/wiki/Category:Icon "Category:Icon")  and  [Unicon](http://rosettacode.org/wiki/Category:Unicon "Category:Unicon")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=44 "Edit section: Icon and Unicon")]

"Works" in both languages:

procedure main()  
    write(0^0)  
end

Output:

->z2z

Run-time error 204
File z2z.icn; Line 2
real overflow, underflow, or division by zero
Traceback:
   main()
   {0 ^ 0} from line 2 in z2z.icn
->

## [J](http://rosettacode.org/wiki/Category:J "Category:J")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=45 "Edit section: J")]

   0 ^ 0  
1

## [Java](http://rosettacode.org/wiki/Category:Java "Category:Java")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=46 "Edit section: Java")]

[System](https://www.google.com/search?hl=en&q=allinurl%3Asystem+java.sun.com&btnI=I%27m%20Feeling%20Lucky).out.println([Math](https://www.google.com/search?hl=en&q=allinurl%3Amath+java.sun.com&btnI=I%27m%20Feeling%20Lucky).pow(0, 0));

Output:

1.0

## [JavaScript](http://rosettacode.org/wiki/Category:JavaScript "Category:JavaScript")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=47 "Edit section: JavaScript")]

### Math.pow[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=48 "Edit section: Math.pow")]

**Works with**:  [Node.js](http://rosettacode.org/wiki/Node.js "Node.js")

In interactive mode:

> Math.pow(0, 0);  
1

### exponentiation operator (**)[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=49 "Edit section: exponentiation operator (**)")]

> 0**0  
1

## [jq](http://rosettacode.org/wiki/Category:Jq "Category:Jq")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=50 "Edit section: jq")]

jq version 1.4 does not have a builtin "power" function. If it were to be defined using the exp and log builtins as 'log * y | exp', then 0 | power(0) would yield null, and therefore a definition that makes a special case of 0^0 should be considered, e.g. along the following lines:

def power(y): y as $y | if $y == 0 then 1 elif . == 0 then 0 else log * $y | exp end;

This definition will however be unsatisfactory for many purposes because it does not maintain precision for integer values of the input (.) and y.

## [Jsish](http://rosettacode.org/wiki/Category:Jsish "Category:Jsish")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=51 "Edit section: Jsish")]

puts(Math.pow(0,0));

Output:

1

## [Julia](http://rosettacode.org/wiki/Category:Julia "Category:Julia")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=52 "Edit section: Julia")]

Try all combinations of complex, float, rational, integer and boolean.

using Printf  
   
const types = (Complex, Float64, Rational, Int, Bool)  
   
for Tb in types, Te in types  
    zb, ze = zero(Tb), zero(Te)  
    r = zb ^ ze  
    @printf("%10s ^ %-10s = %7s ^ %-7s = %-12s (%s)\n", Tb, Te, zb, ze, r, typeof(r))  
end

Output:

   Complex ^ Complex    = 0 + 0im ^ 0 + 0im = 1.0 + 0.0im  (Complex{Float64})
   Complex ^ Float64    = 0 + 0im ^ 0.0     = 1.0 + 0.0im  (Complex{Float64})
   Complex ^ Rational   = 0 + 0im ^ 0//1    = 1.0 + 0.0im  (Complex{Float64})
   Complex ^ Int64      = 0 + 0im ^ 0       = 1 + 0im      (Complex{Int64})
   Complex ^ Bool       = 0 + 0im ^ false   = 1 + 0im      (Complex{Int64})
   Float64 ^ Complex    =     0.0 ^ 0 + 0im = 1.0 + 0.0im  (Complex{Float64})
   Float64 ^ Float64    =     0.0 ^ 0.0     = 1.0          (Float64)
   Float64 ^ Rational   =     0.0 ^ 0//1    = 1.0          (Float64)
   Float64 ^ Int64      =     0.0 ^ 0       = 1.0          (Float64)
   Float64 ^ Bool       =     0.0 ^ false   = 1.0          (Float64)
  Rational ^ Complex    =    0//1 ^ 0 + 0im = 1.0 + 0.0im  (Complex{Float64})
  Rational ^ Float64    =    0//1 ^ 0.0     = 1.0          (Float64)
  Rational ^ Rational   =    0//1 ^ 0//1    = 1.0          (Float64)
  Rational ^ Int64      =    0//1 ^ 0       = 1//1         (Rational{Int64})
  Rational ^ Bool       =    0//1 ^ false   = 1//1         (Rational{Int64})
     Int64 ^ Complex    =       0 ^ 0 + 0im = 1.0 + 0.0im  (Complex{Float64})
     Int64 ^ Float64    =       0 ^ 0.0     = 1.0          (Float64)
     Int64 ^ Rational   =       0 ^ 0//1    = 1.0          (Float64)
     Int64 ^ Int64      =       0 ^ 0       = 1            (Int64)
     Int64 ^ Bool       =       0 ^ false   = 1            (Int64)
      Bool ^ Complex    =   false ^ 0 + 0im = 1.0 + 0.0im  (Complex{Float64})
      Bool ^ Float64    =   false ^ 0.0     = 1.0          (Float64)
      Bool ^ Rational   =   false ^ 0//1    = 1.0          (Float64)
      Bool ^ Int64      =   false ^ 0       = true         (Bool)
      Bool ^ Bool       =   false ^ false   = true         (Bool)

## [K](http://rosettacode.org/wiki/Category:K "Category:K")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=53 "Edit section: K")]

   
  0^0  
1.0  
 

## [Kotlin](http://rosettacode.org/wiki/Category:Kotlin "Category:Kotlin")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=54 "Edit section: Kotlin")]

// version 1.0.6  
   
fun main(args: Array<String>) {  
   println("0 ^ 0 = ${Math.pow(0.0, 0.0)}")  
}

Output:

0 ^ 0 = 1.0

## [Lua](http://rosettacode.org/wiki/Category:Lua "Category:Lua")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=55 "Edit section: Lua")]

No need to try different data types or with / without decimal points as all numbers in Lua are stored in double-precision floating-point format.

print(0^0)

Output:

1

## [M2000 Interpreter](http://rosettacode.org/wiki/Category:M2000_Interpreter "Category:M2000 Interpreter")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=56 "Edit section: M2000 Interpreter")]

M2000 use ** and ^ for power.

   
Module Checkit {  
      x=0  
      y=0  
      Print x**y=1, x^y=1    ' True True  
}  
Checkit  
 

  

## [Maple](http://rosettacode.org/wiki/Category:Maple "Category:Maple")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=57 "Edit section: Maple")]

0^0

Output:

1

However, for consistency with IEEE-754 numerics, we also have a NaN result for the equivalent floating-point exponentiation:

0^0.0

Output:

Float(undefined)

## [Mathematica](http://rosettacode.org/wiki/Category:Mathematica "Category:Mathematica")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=58 "Edit section: Mathematica")]

0^0

Output:

Indeterminate

## [MATLAB](http://rosettacode.org/wiki/Category:MATLAB "Category:MATLAB")  /  [Octave](http://rosettacode.org/wiki/Category:Octave "Category:Octave")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=59 "Edit section: MATLAB / Octave")]

0^0  
[complex](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/complex.html)(0,0)^0

Output:

1
1

## [Mercury](http://rosettacode.org/wiki/Category:Mercury "Category:Mercury")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=60 "Edit section: Mercury")]

:- module zero_to_the_zero_power.  
:- interface.  
   
:- import_module io.  
   
:- pred main(io::di, io::uo) is det.  
   
:- implementation.  
   
:- import_module float, int, integer, list, string.  
   
main(!IO) :-  
   io.format("    int.pow(0, 0) = %d\n", [i(pow(0, 0))], !IO),  
   io.format("integer.pow(zero, zero) = %s\n",  
        [s(to_string(pow(zero, zero)))], !IO),  
   io.format("  float.pow(0.0, 0) = %.1f\n", [f(pow(0.0, 0))], !IO).  
   
:- end_module zero_to_the_zero_power.

Output:

    int.pow(0, 0) = 1
integer.pow(zero, zero) = 1
  float.pow(0.0, 0) = 1.0

## [Microsoft Small Basic](http://rosettacode.org/wiki/Category:Microsoft_Small_Basic "Category:Microsoft Small Basic")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=61 "Edit section: Microsoft Small Basic")]

TextWindow.WriteLine(Math.Power(0,0))

Output:

1

## [min](http://rosettacode.org/wiki/Category:Min "Category:Min")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=62 "Edit section: min")]

**Works with**:  [min](http://rosettacode.org/wiki/Min "Min")  version 0.19.3

0 0 pow puts

Output:

1.0

## [MiniScript](http://rosettacode.org/wiki/Category:MiniScript "Category:MiniScript")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=63 "Edit section: MiniScript")]

print "The result of zero to the zero power is " + 0^0

Output:

The result of zero to the zero power is 1

## [МК-61/52](http://rosettacode.org/wiki/Category:%D0%9C%D0%9A-61/52 "Category:МК-61/52")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=64 "Edit section: МК-61/52")]

Сx	^	x^y	С/П

The result is error message.

## [Neko](http://rosettacode.org/wiki/Category:Neko "Category:Neko")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=65 "Edit section: Neko")]

Neko uses the C math library for exponentiation, Zero to the zero in math.pow(x, y) is treated as being 1.

/**  
 Zero to the zeroth power, in Neko  
*/  
   
var math_pow = $loader.loadprim("std@math_pow", 2)  
   
$print(math_pow(0, 0), "\n")

Output:

prompt$ nekoc zero-to-the-zero.neko
prompt$ neko zero-to-the-zero.n
1

## [NetRexx](http://rosettacode.org/wiki/Category:NetRexx "Category:NetRexx")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=66 "Edit section: NetRexx")]

x=0  
Say '0**0='||x**x

Output:

0**0=1

## [NewLISP](http://rosettacode.org/wiki/Category:NewLISP "Category:NewLISP")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=67 "Edit section: NewLISP")]

([pow](http://www.newlisp.org/downloads/newlisp_manual.html#pow) 0 0)

Output:

1

## [Nial](http://rosettacode.org/wiki/Category:Nial "Category:Nial")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=68 "Edit section: Nial")]

Create an exponentiation table for all type combinations (of integer  `0`, float  `0.0`  and boolean  `o`):

     0 0.0 o outer power 0 0.0 o  
+--+--+--+  
| 1|1.| 1|  
+--+--+--+  
|1.|1.|1.|  
+--+--+--+  
| 1|1.| 1|  
+--+--+--+

## [Nim](http://rosettacode.org/wiki/Category:Nim "Category:Nim")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=69 "Edit section: Nim")]

import math  
   
echo pow(0, 0)

Output:

1.0

## [OCaml](http://rosettacode.org/wiki/Category:OCaml "Category:OCaml")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=70 "Edit section: OCaml")]

In the interpreter:

# 0.0 ** 0.0;;
- : float = 1.
# Complex.pow Complex.zero Complex.zero;;
- : Complex.t = {Complex.re = nan; Complex.im = nan}
# #load "nums.cma";;
# open Num;;
# Int 0 **/ Int 0;;                 
- : Num.num = Int 1

## [Oforth](http://rosettacode.org/wiki/Category:Oforth "Category:Oforth")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=71 "Edit section: Oforth")]

0 0 pow println

Output:

1

## [Ol](http://rosettacode.org/wiki/Category:Ol "Category:Ol")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=72 "Edit section: Ol")]

   
(print "0^0: " (expt 0 0))  
(print "0.0^0: " (expt (inexact 0) 0))  
 

Output:

0^0: 1
0.0^0: 1

## [ooRexx](http://rosettacode.org/wiki/Category:OoRexx "Category:OoRexx")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=73 "Edit section: ooRexx")]

/**********************************************************************  
* 21.04.2014 Walter Pachl  
**********************************************************************/  
Say 'rxCalcpower(0,0)  ->' rxCalcpower(0,0)  
Say '0**0              ->' 0**0  
::requires rxmath library

Output:

rxCalcpower(0,0)  -> 1
0**0              -> 1 

## [PARI/GP](http://rosettacode.org/wiki/Category:PARI/GP "Category:PARI/GP")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=74 "Edit section: PARI/GP")]

0 raised to the power of exact 0 is 0, but 0 cannot be raised to the power of an inexact 0:

0^0  
0.^0  
0^0.

Output:

%1 = 1
%2 = 1
  ***   at top-level: 0^0.
  ***                   ^---
  *** _^_: domain error in gpow(0,n): n <= 0
  ***   Break loop: type 'break' to go back to GP prompt

## [Pascal](http://rosettacode.org/wiki/Category:Pascal "Category:Pascal")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=75 "Edit section: Pascal")]

**Works with**:  [Free Pascal](http://rosettacode.org/wiki/Free_Pascal "Free Pascal")

**Library:**  [math](http://rosettacode.org/mw/index.php?title=Category:Math&action=edit&redlink=1 "Category:Math (page does not exist)")

program ZToZ;  
uses  
  math;  
begin  
  write('0.0 ^ 0 :',IntPower(0.0,0):4:2);  
  writeln('   0.0 ^ 0.0 :',Power(0.0,0.0):4:2);  
end.

output

0.0 ^ 0 :1.00   0.0 ^ 0.0 :1.00

## [Perl](http://rosettacode.org/wiki/Category:Perl "Category:Perl")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=76 "Edit section: Perl")]

[print](https://perldoc.perl.org/functions/print.html) 0 ** 0, "\n";  
   
use Math::Complex;  
   
[print](https://perldoc.perl.org/functions/print.html) cplx(0,0) ** cplx(0,0), "\n";

Output:

1
1

## [Perl 6](http://rosettacode.org/wiki/Category:Perl_6 "Category:Perl 6")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=77 "Edit section: Perl 6")]

**Works with**:  [Rakudo](http://rosettacode.org/wiki/Rakudo "Rakudo")  version 2018.03

say '    type         n      n**n  exp(n,n)';  
say '--------  --------  --------  --------';  
   
for 0, 0.0, FatRat.new(0), 0e0, 0+0i {  
    printf "%8s  %8s  %8s  %8s\n", .^name, $_, $_**$_, exp($_,$_);  
}

Output:

    type         n      n**n  exp(n,n)
--------  --------  --------  --------
     Int         0         1         1
     Rat         0         1         1
  FatRat         0         1         1
     Num         0         1         1
 Complex      0+0i      1+0i      1+0i

## [Phix](http://rosettacode.org/wiki/Category:Phix "Category:Phix")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=78 "Edit section: Phix")]

Fair enough, I have no strong opinions on this matter, so I have just removed the test/error that was present in previous versions. Should you for any reason want to change it back, just edit builtins/VM/pPower.e, search for the two mods dated 3/11/15 (32 and 64 bit, both are two lines, test eax/rax; jz :e102cr0tple0), save and rebuild (run "p -c p"), which should take less than 10 seconds.

?power(0,0)

Output:

1

## [PHP](http://rosettacode.org/wiki/Category:PHP "Category:PHP")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=79 "Edit section: PHP")]

<?php  
echo [pow](http://www.php.net/pow)(0,0);  
echo 0 ** 0; // PHP 5.6+ only  
?>

Output:

1
1

## [PicoLisp](http://rosettacode.org/wiki/Category:PicoLisp "Category:PicoLisp")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=80 "Edit section: PicoLisp")]

   
(** 0 0)  
 

Output:

1

## [PL/I](http://rosettacode.org/wiki/Category:PL/I "Category:PL/I")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=81 "Edit section: PL/I")]

 zhz: Proc Options(Main);  
 Dcl a dec float(10) Init(1);  
 Dcl b dec float(10) Init(0);  
 Put skip list('1**0=',a**b);  
 Put skip list('0**1=',b**a);  
 Put skip list('0**0=',b**b);  
 End;

Output:

1**0=                    1.000000000E+0000
0**1=                    0.000000000E+0000
0**0=
IBM0682I  ONCODE=1553  X in EXPONENT(X) was invalid.
   At offset +0000025B in procedure with entry ZHZ   

## [PowerShell](http://rosettacode.org/wiki/Category:PowerShell "Category:PowerShell")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=82 "Edit section: PowerShell")]

Write-Host "0 ^ 0 = " ([math]::pow(0,0))

Output :

0 ^ 0 =  1

## [PureBasic](http://rosettacode.org/wiki/Category:PureBasic "Category:PureBasic")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=83 "Edit section: PureBasic")]

   
If OpenConsole()  
  PrintN("Zero to the zero power is " + Pow(0,0))  
  PrintN("")  
  PrintN("Press any key to close the console")  
  Repeat: Delay(10) : Until Inkey() <> ""  
  CloseConsole()  
EndIf  
 

Output:

Zero to the zero power is 1

  

## [Pyret](http://rosettacode.org/wiki/Category:Pyret "Category:Pyret")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=84 "Edit section: Pyret")]

num-expt(0, 0)

Output:

1

## [Python](http://rosettacode.org/wiki/Category:Python "Category:Python")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=85 "Edit section: Python")]

### Python3[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=86 "Edit section: Python3")]

from decimal import Decimal  
from fractions import Fraction  
from itertools import product  
   
zeroes = [0, 0.0, 0j, Decimal(0), Fraction(0, 1), -0.0, -0.0j, Decimal(-0.0)]  
for i, j in product(zeroes, repeat=2):  
    try:  
        ans = i**j  
    except:  
        ans = '<Exception raised>'  
    print(f'{i!r:>15} ** {j!r:<15} = {ans!r}')

Output:

              0 ** 0               = 1
              0 ** 0.0             = 1.0
              0 ** 0j              = (1+0j)
              0 ** Decimal('0')    = '<Exception raised>'
              0 ** Fraction(0, 1)  = 1
              0 ** -0.0            = 1.0
              0 ** (-0-0j)         = (1+0j)
              0 ** Decimal('-0')   = '<Exception raised>'
            0.0 ** 0               = 1.0
            0.0 ** 0.0             = 1.0
            0.0 ** 0j              = (1+0j)
            0.0 ** Decimal('0')    = '<Exception raised>'
            0.0 ** Fraction(0, 1)  = 1.0
            0.0 ** -0.0            = 1.0
            0.0 ** (-0-0j)         = (1+0j)
            0.0 ** Decimal('-0')   = '<Exception raised>'
             0j ** 0               = (1+0j)
             0j ** 0.0             = (1+0j)
             0j ** 0j              = (1+0j)
             0j ** Decimal('0')    = '<Exception raised>'
             0j ** Fraction(0, 1)  = (1+0j)
             0j ** -0.0            = (1+0j)
             0j ** (-0-0j)         = (1+0j)
             0j ** Decimal('-0')   = '<Exception raised>'
   Decimal('0') ** 0               = '<Exception raised>'
   Decimal('0') ** 0.0             = '<Exception raised>'
   Decimal('0') ** 0j              = '<Exception raised>'
   Decimal('0') ** Decimal('0')    = '<Exception raised>'
   Decimal('0') ** Fraction(0, 1)  = '<Exception raised>'
   Decimal('0') ** -0.0            = '<Exception raised>'
   Decimal('0') ** (-0-0j)         = '<Exception raised>'
   Decimal('0') ** Decimal('-0')   = '<Exception raised>'
 Fraction(0, 1) ** 0               = Fraction(1, 1)
 Fraction(0, 1) ** 0.0             = 1.0
 Fraction(0, 1) ** 0j              = (1+0j)
 Fraction(0, 1) ** Decimal('0')    = '<Exception raised>'
 Fraction(0, 1) ** Fraction(0, 1)  = Fraction(1, 1)
 Fraction(0, 1) ** -0.0            = 1.0
 Fraction(0, 1) ** (-0-0j)         = (1+0j)
 Fraction(0, 1) ** Decimal('-0')   = '<Exception raised>'
           -0.0 ** 0               = 1.0
           -0.0 ** 0.0             = 1.0
           -0.0 ** 0j              = (1+0j)
           -0.0 ** Decimal('0')    = '<Exception raised>'
           -0.0 ** Fraction(0, 1)  = 1.0
           -0.0 ** -0.0            = 1.0
           -0.0 ** (-0-0j)         = (1+0j)
           -0.0 ** Decimal('-0')   = '<Exception raised>'
        (-0-0j) ** 0               = (1+0j)
        (-0-0j) ** 0.0             = (1+0j)
        (-0-0j) ** 0j              = (1+0j)
        (-0-0j) ** Decimal('0')    = '<Exception raised>'
        (-0-0j) ** Fraction(0, 1)  = (1+0j)
        (-0-0j) ** -0.0            = (1+0j)
        (-0-0j) ** (-0-0j)         = (1+0j)
        (-0-0j) ** Decimal('-0')   = '<Exception raised>'
  Decimal('-0') ** 0               = '<Exception raised>'
  Decimal('-0') ** 0.0             = '<Exception raised>'
  Decimal('-0') ** 0j              = '<Exception raised>'
  Decimal('-0') ** Decimal('0')    = '<Exception raised>'
  Decimal('-0') ** Fraction(0, 1)  = '<Exception raised>'
  Decimal('-0') ** -0.0            = '<Exception raised>'
  Decimal('-0') ** (-0-0j)         = '<Exception raised>'
  Decimal('-0') ** Decimal('-0')   = '<Exception raised>'

### Python2[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=87 "Edit section: Python2")]

from decimal import Decimal  
from fractions import Fraction  
for n in (Decimal(0), Fraction(0, 1), complex(0), float(0), int(0)):  
	try:  
		n1 = n**n  
	except:  
		n1 = '<Raised exception>'  
	try:  
		n2 = pow(n, n)  
	except:  
		n2 = '<Raised exception>'  
	print('%8s: ** -> %r; pow -> %r' % (n.__class__.__name__, n1, n2))

Output:

 Decimal: ** -> '<Raised exception>'; pow -> '<Raised exception>'
Fraction: ** -> Fraction(1, 1); pow -> Fraction(1, 1)
 complex: ** -> (1+0j); pow -> (1+0j)
   float: ** -> 1.0; pow -> 1.0
     int: ** -> 1; pow -> 1

## [R](http://rosettacode.org/wiki/Category:R "Category:R")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=88 "Edit section: R")]

[print](http://stat.ethz.ch/R-manual/R-devel/library/base/html/print.html)(0^0)

Output:

1

## [Racket](http://rosettacode.org/wiki/Category:Racket "Category:Racket")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=89 "Edit section: Racket")]

#lang racket  
;; as many zeros as I can think of...  
(define zeros (list  
               0  ; unspecified number type  
               0. ; hinted as float  
               #e0 ; explicitly exact  
               #i0 ; explicitly inexact  
               0+0i ; exact complex  
               0.+0.i ; float inexact  
               ))  
(for*((z zeros) (p zeros))  
  (printf "(~a)^(~a) = ~s~%" z p  
  (with-handlers [(exn:fail:contract:divide-by-zero? exn-message)]  
    (expt z p))))

Output:

(0)^(0) = 1
(0)^(0.0) = 1.0
(0)^(0) = 1
(0)^(0.0) = 1.0
(0)^(0) = 1
(0)^(0.0+0.0i) = "expt: undefined for 0 and 0.0+0.0i"
(0.0)^(0) = 1
(0.0)^(0.0) = 1.0
(0.0)^(0) = 1
(0.0)^(0.0) = 1.0
(0.0)^(0) = 1
(0.0)^(0.0+0.0i) = +nan.0+nan.0i
(0)^(0) = 1
(0)^(0.0) = 1.0
(0)^(0) = 1
(0)^(0.0) = 1.0
(0)^(0) = 1
(0)^(0.0+0.0i) = "expt: undefined for 0 and 0.0+0.0i"
(0.0)^(0) = 1
(0.0)^(0.0) = 1.0
(0.0)^(0) = 1
(0.0)^(0.0) = 1.0
(0.0)^(0) = 1
(0.0)^(0.0+0.0i) = +nan.0+nan.0i
(0)^(0) = 1
(0)^(0.0) = 1.0
(0)^(0) = 1
(0)^(0.0) = 1.0
(0)^(0) = 1
(0)^(0.0+0.0i) = "expt: undefined for 0 and 0.0+0.0i"
(0.0+0.0i)^(0) = 1
(0.0+0.0i)^(0.0) = 1.0+0.0i
(0.0+0.0i)^(0) = 1
(0.0+0.0i)^(0.0) = 1.0+0.0i
(0.0+0.0i)^(0) = 1
(0.0+0.0i)^(0.0+0.0i) = +nan.0+nan.0i

## [REXX](http://rosettacode.org/wiki/Category:REXX "Category:REXX")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=90 "Edit section: REXX")]

/*REXX program shows the results of  raising zero  to the  zeroth power.*/  
say  '0 ** 0  (zero to the zeroth power) ───► '    0**0

  
using PC/REXX  
using Personal REXX  
using REGINA  
using ooRexx

Output:

0 ** 0  (zero to the zeroth power) ───►  1

using R4

Output:

Error 26 : Invalid whole number (SYNTAX)
Information: 0 ** 0 is undefined
Error occurred in statement# 2
Statement source: say '0 ** 0  (zero to the zeroth power) ───► ' 0**0
Statement context: C:\ZERO_TO0.REX, procedure: ZERO_TO0

using ROO

Output:

Error 26 : Invalid whole number (SYNTAX)
Information: 0 ** 0 is undefined
Error occurred in statement# 2
Statement source: say '0 ** 0  (zero to the zeroth power) ───► ' 0**0
Statement context: C:\ZERO_TO0.REX, procedure: ZERO_TO0

## [Ring](http://rosettacode.org/wiki/Category:Ring "Category:Ring")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=91 "Edit section: Ring")]

   
x = 0  
y = 0  
z = pow(x,y)  
see "z=" + z + nl   # z=1  
 

## [Ruby](http://rosettacode.org/wiki/Category:Ruby "Category:Ruby")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=92 "Edit section: Ruby")]

require 'bigdecimal'  
   
[0, 0.0, Complex(0), Rational(0), BigDecimal.new("0")].each do |n|  
  printf "%10s: ** -> %s\n" % [n.class, n**n]  
end

Output:

    Fixnum: ** -> 1
     Float: ** -> 1.0
   Complex: ** -> 1+0i
  Rational: ** -> 1/1
BigDecimal: ** -> 0.1E1

## [Rust](http://rosettacode.org/wiki/Category:Rust "Category:Rust")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=93 "Edit section: Rust")]

fn main() {  
    println!("{}",0u32.pow(0));  
}

Output:

1

  

## [S-lang](http://rosettacode.org/wiki/Category:S-lang "Category:S-lang")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=94 "Edit section: S-lang")]

print(0^0);

Output:

1.0

## [Scala](http://rosettacode.org/wiki/Category:Scala "Category:Scala")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=95 "Edit section: Scala")]

**Library:**  [Scala](http://rosettacode.org/wiki/Category:Scala "Category:Scala")

  assert(math.pow(0, 0) == 1, "Scala blunder, should go back to school !")

## [Scheme](http://rosettacode.org/wiki/Category:Scheme "Category:Scheme")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=96 "Edit section: Scheme")]

(display (expt 0 0)) (newline)  
(display (expt 0.0 0.0)) (newline)  
(display (expt 0+0i 0+0i)) (newline)

Output:

1
1.0
1.0

## [Seed7](http://rosettacode.org/wiki/Category:Seed7 "Category:Seed7")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=97 "Edit section: Seed7")]

$ include "seed7_05.s7i";  
  include "float.s7i";  
  include "complex.s7i";  
   
const proc: main is func  
  begin  
    writeln("0      ** 0   = " <& 0 ** 0);  
    writeln("0.0    ** 0   = " <& 0.0 ** 0);  
    writeln("0.0    ** 0.0 = " <& 0.0 ** 0.0);  
    writeln("0.0+0i ** 0   = " <& complex(0.0) ** 0);  
  end func;  
 

Output:

0      ** 0   = 1
0.0    ** 0   = 1.0
0.0    ** 0.0 = 1.0
0.0+0i ** 0   = 1.0+0.0i

## [Sidef](http://rosettacode.org/wiki/Category:Sidef "Category:Sidef")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=98 "Edit section: Sidef")]

[0, Complex(0, 0)].each {|n|  
    say n**n  
}

Output:

1
1

Taking the 0'th root of a number and raising it back to the zero power, we also get a 1:

say 0.root(0).pow(0)       # => 1  
say ((0**(1/0))**0)        # => 1

## [Sinclair ZX81 BASIC](http://rosettacode.org/wiki/Category:Sinclair_ZX81_BASIC "Category:Sinclair ZX81 BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=99 "Edit section: Sinclair ZX81 BASIC")]

PRINT 0**0

Output:

1

## [Smalltalk](http://rosettacode.org/wiki/Category:Smalltalk "Category:Smalltalk")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=100 "Edit section: Smalltalk")]

   
0 raisedTo: 0   
0.0 raisedTo: 0.0   
 

Output:

1
1.0

  

## [smart BASIC](http://rosettacode.org/wiki/Category:Smart_BASIC "Category:Smart BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=101 "Edit section: smart BASIC")]

[PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) 0^0

Output:

1

## [SQL](http://rosettacode.org/wiki/Category:SQL "Category:SQL")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=102 "Edit section: SQL")]

   
SQL> SELECT POWER(0,0) FROM dual;  
 

Output:

POWER(0,0)
----------
         1

## [Standard ML](http://rosettacode.org/wiki/Category:Standard_ML "Category:Standard ML")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=103 "Edit section: Standard ML")]

In the interpreter:

- Math.pow (0.0, 0.0);
val it = 1.0 : real

## [Stata](http://rosettacode.org/wiki/Category:Stata "Category:Stata")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=104 "Edit section: Stata")]

. display 0^0  
1

## [Swift](http://rosettacode.org/wiki/Category:Swift "Category:Swift")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=105 "Edit section: Swift")]

import Darwin  
print(pow(0.0,0.0))

Output:

1.0

## [Tcl](http://rosettacode.org/wiki/Category:Tcl "Category:Tcl")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=106 "Edit section: Tcl")]

Interactively…

% expr 0**0  
1  
% expr 0.0**0.0  
1.0

## [TI-83_BASIC](http://rosettacode.org/wiki/Category:TI-83_BASIC "Category:TI-83 BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=107 "Edit section: TI-83 BASIC")]

0^0

Output:

ERROR:DOMAIN

## [uBasic/4tH](http://rosettacode.org/wiki/Category:UBasic/4tH "Category:UBasic/4tH")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=108 "Edit section: uBasic/4tH")]

Print 0^0

Output:

1

0 OK, 0:9

## [Ursa](http://rosettacode.org/wiki/Category:Ursa "Category:Ursa")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=109 "Edit section: Ursa")]

Cygnus/X Ursa is written in Java, and as a result returns 1.0 when raising 0 to the 0.

> out (pow 0 0) endl console  
1.0

## [VBA](http://rosettacode.org/wiki/Category:VBA "Category:VBA")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=110 "Edit section: VBA")]

Public Sub zero()  
    x = 0  
    y = 0  
    z = 0 ^ 0  
    Debug.Print "z ="; z  
End Sub

Output:

z = 1

## [VBScript](http://rosettacode.org/wiki/Category:VBScript "Category:VBScript")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=111 "Edit section: VBScript")]

WScript.Echo 0 ^ 0

Output:

1

## [Visual Basic .NET](http://rosettacode.org/wiki/Category:Visual_Basic_.NET "Category:Visual Basic .NET")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=112 "Edit section: Visual Basic .NET")]

Module Program  
    Sub Main()  
        Console.Write(0^0)  
    End Sub  
End Module

Output:

1

## [XLISP](http://rosettacode.org/wiki/Category:XLISP "Category:XLISP")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=113 "Edit section: XLISP")]

XLISP 3.3, September 6, 2002 Copyright (c) 1984-2002, by David Betz  
[1] (expt 0 0)  
   
1  
[2] 

## [zkl](http://rosettacode.org/wiki/Category:Zkl "Category:Zkl")[[edit](http://rosettacode.org/mw/index.php?title=Zero_to_the_zero_power&action=edit&section=114 "Edit section: zkl")]

(0.0).pow(0)  //--> 1.0  
var BN=Import("zklBigNum"); // big ints  
BN(0).pow(0) //--> 1

[Categories](http://rosettacode.org/wiki/Special:Categories "Special:Categories"):

-   [Programming Tasks](http://rosettacode.org/wiki/Category:Programming_Tasks "Category:Programming Tasks")
-   [Solutions by Programming Task](http://rosettacode.org/wiki/Category:Solutions_by_Programming_Task "Category:Solutions by Programming Task")
-   [Simple](http://rosettacode.org/wiki/Category:Simple "Category:Simple")
-   [8th](http://rosettacode.org/wiki/Category:8th "Category:8th")
-   [ARM Assembly](http://rosettacode.org/wiki/Category:ARM_Assembly "Category:ARM Assembly")
-   [ARM Assembly/Omit](http://rosettacode.org/mw/index.php?title=Category:ARM_Assembly/Omit&action=edit&redlink=1 "Category:ARM Assembly/Omit (page does not exist)")
-   [AutoHotkey](http://rosettacode.org/wiki/Category:AutoHotkey "Category:AutoHotkey")
-   [Ada](http://rosettacode.org/wiki/Category:Ada "Category:Ada")
-   [ALGOL 68](http://rosettacode.org/wiki/Category:ALGOL_68 "Category:ALGOL 68")
-   [APL](http://rosettacode.org/wiki/Category:APL "Category:APL")