
# Walk a directory/Non-recursively

[![Task](http://rosettacode.org/mw/images/thumb/b/ba/Rcode-button-task-crushed.png/64px-Rcode-button-task-crushed.png)](http://rosettacode.org/wiki/Category:Solutions_by_Programming_Task "Category:Solutions by Programming Task")

**Walk a directory/Non-recursively**  
You are encouraged to  [solve this task](http://rosettacode.org/wiki/Rosetta_Code:Solve_a_Task "Rosetta Code:Solve a Task")  according to the task description, using any language you may know.

Task

Walk a given directory and print the  _names_  of files matching a given pattern.

(How is "pattern" defined? substring match? DOS pattern? BASH pattern? ZSH pattern? Perl regular expression?)

  
**Note:**  This task is for non-recursive methods. These tasks should read a  _single directory_, not an entire directory tree.

**Note:**  Please be careful when running any code presented here.

  

Related task

-   [Walk Directory Tree](http://rosettacode.org/wiki/Walk_Directory_Tree "Walk Directory Tree")  (read entire directory tree).

  
  

## Contents

[[hide](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#)]

-   [1  68000 Assembly](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#68000_Assembly)
-   [2  8th](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#8th)
-   [3  Ada](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Ada)
-   [4  ALGOL 68](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#ALGOL_68)
-   [5  AppleScript](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#AppleScript)
-   [6  AutoHotkey](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#AutoHotkey)
-   [7  BaCon](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#BaCon)
-   [8  BASIC](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#BASIC)
-   [9  Batch File](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Batch_File)
-   [10  BBC BASIC](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#BBC_BASIC)
-   [11  C](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#C)
-   [12  C#](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#C.23)
-   [13  C++](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#C.2B.2B)
-   [14  Clojure](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Clojure)
-   [15  ColdFusion](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#ColdFusion)
-   [16  Common Lisp](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Common_Lisp)
-   [17  D](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#D)
-   [18  DCL](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#DCL)
-   [19  E](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#E)
-   [20  Elena](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Elena)
-   [21  Elixir](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Elixir)
-   [22  Emacs Lisp](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Emacs_Lisp)
-   [23  Erlang](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Erlang)
-   [24  Euphoria](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Euphoria)
-   [25  F#](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#F.23)
-   [26  Factor](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Factor)
-   [27  Forth](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Forth)
-   [28  Gambas](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Gambas)
-   [29  Go](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Go)
-   [30  Groovy](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Groovy)
-   [31  Haskell](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Haskell)
-   [32  HicEst](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#HicEst)
-   [33  Icon and Unicon](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Icon_and_Unicon)
-   [34  IDL](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#IDL)
-   [35  J](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#J)
-   [36  Java](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Java)
-   [37  JavaScript](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#JavaScript)
-   [38  Julia](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Julia)
-   [39  Kotlin](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Kotlin)
-   [40  Lasso](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Lasso)
-   [41  Lingo](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Lingo)
-   [42  LiveCode](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#LiveCode)
-   [43  Lua](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Lua)
-   [44  M2000 Interpreter](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#M2000_Interpreter)
-   [45  Mathematica](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Mathematica)
-   [46  MAXScript](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#MAXScript)
-   [47  Nemerle](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Nemerle)
-   [48  NetRexx](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#NetRexx)
-   [49  Nim](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Nim)
-   [50  Objeck](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Objeck)
-   [51  Objective-C](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Objective-C)
-   [52  OCaml](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#OCaml)
-   [53  Oz](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Oz)
-   [54  Pascal](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Pascal)
-   [55  Perl](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Perl)
-   [56  Perl 6](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Perl_6)
-   [57  Phix](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Phix)
-   [58  PHP](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#PHP)
-   [59  PicoLisp](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#PicoLisp)
-   [60  Pike](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Pike)
-   [61  Pop11](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Pop11)
-   [62  PowerShell](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#PowerShell)
-   [63  PureBasic](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#PureBasic)
-   [64  Python](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Python)
-   [65  R](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#R)
-   [66  Racket](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Racket)
-   [67  Rascal](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Rascal)
-   [68  Raven](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Raven)
-   [69  REXX](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#REXX)
-   [70  Ring](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Ring)
-   [71  Ruby](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Ruby)
-   [72  Run BASIC](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Run_BASIC)
-   [73  Rust](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Rust)
-   [74  Scala](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Scala)
-   [75  Seed7](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Seed7)
-   [76  Sidef](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Sidef)
-   [77  Smalltalk](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Smalltalk)
-   [78  Tcl](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Tcl)
-   [79  Toka](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Toka)
-   [80  TUSCRIPT](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#TUSCRIPT)
-   [81  TXR](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#TXR)
    -   [81.1  Using glob](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Using_glob)
    -   [81.2  Using open-directory and get-lines](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Using_open-directory_and_get-lines)
-   [82  UNIX Shell](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#UNIX_Shell)
-   [83  UnixPipes](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#UnixPipes)
-   [84  VBScript](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#VBScript)
-   [85  Visual Basic .NET](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Visual_Basic_.NET)
-   [86  zkl](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#zkl)
-   [87  Zsh](http://rosettacode.org/wiki/Walk_a_directory/Non-recursively#Zsh)

## [68000 Assembly](http://rosettacode.org/wiki/Category:68000_Assembly "Category:68000 Assembly")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=1 "Edit section: 68000 Assembly")]

Non-recursive directory walk in Motorola 68000 assembly language under AmigaOs 2.04+ by Thorham. Uses regular Amiga dos pattern matching.

;  
; Non-recursive directory walk for Motorola 68000 under AmigaOs 2.04+ by Thorham  
;  
   
execBase equ 4  
   
;  
; from exec includes  
;  
_LVOOpenLibrary equ -552  
_LVOCloseLibrary equ -414  
_LVOAllocVec equ -684  
_LVOFreeVec equ -690  
   
MEMF_ANY equ 0  
   
;  
; from dos includes  
;  
_LVOVPrintf equ -954  
_LVOExamine equ -102  
_LVOExNext equ -108  
_LVOLock equ -84  
_LVOUnLock equ -90  
_LVOParsePatternNoCase equ -966  
_LVOMatchPatternNoCase equ -972  
   
ACCESS_READ equ -2  
                    rsset   0  
fib_DiskKey         rs.l    1  
fib_DirEntryType    rs.l    1  
fib_FileName        rs.b    108  
fib_Protection      rs.l    1  
fib_EntryType       rs.l    1  
fib_Size            rs.l    1  
fib_NumBlocks       rs.l    1  
fib_DateStamp       rs.b    12  
fib_Comment         rs.b    80  
fib_OwnerUID        rs.w    1  
fib_OwnerGID        rs.w    1  
fib_Reserved        rs.b    32  
fib_SIZEOF          rs.b    0  
   
;  
; main  
;  
   
start  
    move.l  execBase,a6  
   
; open dos.library  
   
    lea     dosName,a1  
    moveq   #37,d0  
    jsr     _LVOOpenLibrary(a6)  
    move.l  d0,dosBase  
    beq     exit  
   
; allocate memory for file info block  
   
    move.l  #fib_SIZEOF,d0  
    move.l  #MEMF_ANY,d1  
    jsr     _LVOAllocVec(a6)  
    move.l  d0,fib  
    beq     exit  
   
; get directory lock  
   
    move.l  dosBase,a6  
   
    move.l  #pathString,d1  
    move.l  #ACCESS_READ,d2  
    jsr     _LVOLock(a6)  
    move.l  d0,lock  
    beq     exit  
   
; examine directory for ExNext  
   
    move.l  lock,d1  
    move.l  fib,d2  
    jsr     _LVOExamine(a6)  
    tst.w   d0  
    beq     exit  
   
; parse pattern string  
   
    move.l  #patternString,d1  
    move.l  #patternParsed,d2  
    move.l  #sizeof_patternString*2+2,d3  
    jsr     _LVOParsePatternNoCase(a6)  
    tst.l   d0  
    blt     exit  
   
; get some pointers for use in the loop  
   
    lea     printfArgs,a2  
    move.l  fib,a3  
    lea     fib_FileName(a3),a3  
   
.loop  
  
; get next directory entry  
   
    move.l  lock,d1  
    move.l  fib,d2  
    jsr     _LVOExNext(a6)  
    tst.w   d0  
    beq     exit  
   
; match pattern  
   
    move.l  #patternParsed,d1  
    move.l  a3,d2  
    jsr     _LVOMatchPatternNoCase(a6)  
   
; if match then print file name  
   
    tst.l   d0  
    beq     .nomatch  
   
    move.l  a3,(a2)  
    move.l  #formatString,d1  
    move.l  #printfArgs,d2  
    jsr     _LVOVPrintf(a6)  
   
.nomatch  
    bra     .loop  
   
; cleanup and exit  
   
exit  
    move.l  dosBase,a6  
    move.l  lock,d1  
    jsr     _LVOUnLock(a6)  
   
    move.l  execBase,a6  
    move.l  fib,a1  
    tst.l   a1  
    beq     .l1  
    jsr     _LVOFreeVec(a6)  
.l1  
    move.l  dosBase,a1  
    jsr     _LVOCloseLibrary(a6)  
    rts  
   
    section data,data_p  
;  
; variables  
;  
dosBase  
    dc.l    0  
   
lock  
    dc.l    0  
   
fib  
    dc.l    0  
   
printfArgs  
    dc.l    0  
;  
; strings  
;  
dosName  
    dc.b    "dos.library",0  
   
pathString  
    dc.b    "ram:",0  
   
formatString  
    dc.b    "%s",10,0  
   
patternString  
    dc.b    "#?",0  
patternString_end  
sizeof_patternString=patternString_end-patternString  
   
patternParsed  
    dcb.b   sizeof_patternString*2+2

## [8th](http://rosettacode.org/wiki/Category:8th "Category:8th")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=2 "Edit section: 8th")]

   
"*.c" f:glob \ puts an array of strings with the file names on the top of the stack  
 

## [Ada](http://rosettacode.org/wiki/Category:Ada "Category:Ada")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=3 "Edit section: Ada")]

**Works with**:  [GCC](http://rosettacode.org/wiki/GCC "GCC")  version 4.12

with Ada.Directories; use Ada.Directories;  
with Ada.Text_IO; use Ada.Text_IO;  
   
procedure Walk_Directory  
            (Directory : in String := ".";  
             Pattern   : in String := "") -- empty pattern = all file names/subdirectory names  
is  
   Search  : Search_Type;  
   Dir_Ent : Directory_Entry_Type;  
begin  
   Start_Search (Search, Directory, Pattern);  
   
   while More_Entries (Search) loop  
      Get_Next_Entry (Search, Dir_Ent);  
      Put_Line (Simple_Name (Dir_Ent));  
   end loop;  
   
   End_Search (Search);  
end Walk_Directory;

## [ALGOL 68](http://rosettacode.org/wiki/Category:ALGOL_68 "Category:ALGOL 68")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=4 "Edit section: ALGOL 68")]

**Works with**:  [ALGOL 68G](http://rosettacode.org/wiki/ALGOL_68G "ALGOL 68G")  version Any - tested with release mk15-0.8b.fc9.i386 - uses non-standard library routines  _get directory_  and _grep in string_.

INT match=0, no match=1, out of memory error=2, other error=3;  
   
[]STRING directory = get directory(".");  
FOR file index TO UPB directory DO  
  STRING file = directory[file index];  
  IF grep in string("[Ss]ort*.[.]a68$", file, NIL, NIL) = match THEN  
    print((file, new line))  
  FI  
OD

Sample output:

Quick_sort.a68
Shell_sort.a68
Cocktail_Sort.a68
Selection_Sort.a68
Merge_sort.a68
Bobosort.a68
Insertion_Sort.a68
Permutation_Sort.a68

## [AppleScript](http://rosettacode.org/wiki/Category:AppleScript "Category:AppleScript")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=5 "Edit section: AppleScript")]

AppleScript itself has limited built-in file system access. Typically, the Mac OS Finder is used to gather such information. To list all file/folders in the root directory:

tell application "Finder" to return name of every item in (startup disk)  
--> EXAMPLE RESULT: {"Applications", "Developer", "Library", "System", "Users"}

To list all pdf files in user's home directory:

tell application "Finder" to return name of every item in (path to documents folder from user domain) whose name ends with "pdf"  
--> EXAMPLE RESULT: {"About Stacks.pdf", "Test.pdf"}

The key clause is the  `whose`  modifier keyword. The Finder can interpret many variations, including such terms as  `whose name begins with`,  `whose name contains`, etc. As well as boolean combinations:

tell application "Finder" to return name of every item in (path to documents folder from user domain) whose name does not contain "about" and name ends with "pdf"  
--> RETURNS: {"Test.pdf"}

The Finder also supports the  `entire contents`  modifier keyword, which effectively performs a recursive directory scan without recursion.

tell application "Finder" to return name of every item in entire contents of (path to documents folder from user domain) whose name ends with "pdf"

## [AutoHotkey](http://rosettacode.org/wiki/Category:AutoHotkey "Category:AutoHotkey")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=6 "Edit section: AutoHotkey")]

Display all INI files in Windows directory.

[Loop](http://www.autohotkey.com/docs/commands/Loop.htm), %A_WinDir%\*.ini  
 out .= [A_LoopFileName](http://www.autohotkey.com/docs/Variables.htm#A_LoopFileName) "`n"  
[MsgBox](http://www.autohotkey.com/docs/commands/MsgBox.htm),% out

## [BaCon](http://rosettacode.org/wiki/Category:BaCon "Category:BaCon")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=7 "Edit section: BaCon")]

This code will print all files in the current directory ".", separated by a newline symbol:

[PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) WALK$(".", 1, ".+", FALSE, NL$)

## [BASIC](http://rosettacode.org/wiki/Category:BASIC "Category:BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=8 "Edit section: BASIC")]

**Works with**:  [QuickBASIC](http://rosettacode.org/wiki/QuickBASIC "QuickBASIC")  version 7

(older versions don't have  `DIR$`)

DOS wildcards are rather underpowered when compared to... well... anything else.

[DECLARE](http://www.qbasicnews.com/qboho/qckdeclare.shtml) SUB show (pattern [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [STRING](http://www.qbasicnews.com/qboho/qckstring.shtml))  
   
show "*.*"  
   
SUB show (pattern [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [STRING](http://www.qbasicnews.com/qboho/qckstring.shtml))  
    [DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) f [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [STRING](http://www.qbasicnews.com/qboho/qckstring.shtml)  
    f = DIR$(pattern)  
    DO WHILE [LEN](http://www.qbasicnews.com/qboho/qcklen.shtml)(f)  
        [PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) f  
        f = DIR$  
    LOOP  
[END](http://www.qbasicnews.com/qboho/qckend.shtml) SUB

## [Batch File](http://rosettacode.org/wiki/Category:Batch_File "Category:Batch File")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=9 "Edit section: Batch File")]

A simple command that displays all EXE files in System32 directory non-recursively.

[dir](https://www.ss64.com/nt/dir.html) /b "%windir%\system32\*.exe"

The same command inside FOR loop:

-   Inside a Batch File:

@[for](https://www.ss64.com/nt/for.html) /F "tokens=*" %%F [in](https://www.ss64.com/nt/in.html) ('[dir](https://www.ss64.com/nt/dir.html) /b "%windir%\system32\*.exe"') [do](https://www.ss64.com/nt/do.html) [echo](https://www.ss64.com/nt/echo.html) %%F

-   Command-line:

[for](https://www.ss64.com/nt/for.html) /F "tokens=*" %F [in](https://www.ss64.com/nt/in.html) ('[dir](https://www.ss64.com/nt/dir.html) /b "%windir%\system32\*.exe"') [do](https://www.ss64.com/nt/do.html) [echo](https://www.ss64.com/nt/echo.html) %F

## [BBC BASIC](http://rosettacode.org/wiki/Category:BBC_BASIC "Category:BBC BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=10 "Edit section: BBC BASIC")]

**Works with**:  [BBC BASIC for Windows](http://rosettacode.org/wiki/BBC_BASIC_for_Windows "BBC BASIC for Windows")

      directory$ = "C:\Windows\"  
      pattern$ = "*.ini"  
      PROClistdir(directory$ + pattern$)  
      END  
   
      DEF PROClistdir(afsp$)  
      LOCAL dir%, sh%, res%  
      DIM dir% LOCAL 317  
      SYS "FindFirstFile", afsp$, dir% TO sh%  
      IF sh% <> -1 THEN  
        REPEAT  
          PRINT $$(dir%+44)  
          SYS "FindNextFile", sh%, dir% TO res%  
        UNTIL res% = 0  
        SYS "FindClose", sh%  
      ENDIF  
      ENDPROC

## [C](http://rosettacode.org/wiki/Category:C "Category:C")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=11 "Edit section: C")]

**Library:**  [POSIX](http://rosettacode.org/wiki/Category:POSIX "Category:POSIX")

**Works with**:  [POSIX](http://rosettacode.org/wiki/POSIX "POSIX")  version .1-2001

In this example, the pattern is a  [POSIX](http://rosettacode.org/wiki/POSIX "POSIX")  extended regular expression.

#include <sys/types.h>  
#include <dirent.h>  
#include <regex.h>  
#include <stdio.h>  
   
enum {  
    WALK_OK = 0,  
    WALK_BADPATTERN,  
    WALK_BADOPEN,  
};  
   
int walker(const char *dir, const char *pattern)  
{  
    struct dirent *entry;  
    regex_t reg;  
    DIR *d;   
   
    if (regcomp(&reg, pattern, REG_EXTENDED | REG_NOSUB))  
        return WALK_BADPATTERN;  
    if (!(d = opendir(dir)))  
        return WALK_BADOPEN;  
    while (entry = readdir(d))  
        if (!regexec(&reg, entry->d_name, 0, NULL, 0))  
            [puts](https://www.opengroup.org/onlinepubs/009695399/functions/puts.html)(entry->d_name);  
    closedir(d);  
    regfree(&reg);  
    return WALK_OK;  
}  
   
int main()  
{  
    walker(".", ".\\.c$");  
    return 0;  
}

## [C#](http://rosettacode.org/wiki/Category:C_sharp "Category:C sharp")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=12 "Edit section: C#")]

using System;  
using System.IO;  
   
namespace DirectoryWalk  
{  
    class Program  
    {  
        static void Main(string[] args)  
        {  
            string[] filePaths = Directory.GetFiles(@"c:\MyDir", "a*");  
            foreach (string filename in filePaths)  
                Console.WriteLine(filename);              
        }  
    }  
}  
 

## [C++](http://rosettacode.org/wiki/Category:C%2B%2B "Category:C++")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=13 "Edit section: C++")]

**Library:**  [boost](http://rosettacode.org/wiki/Category:Boost "Category:Boost") **version**  1.50.0

#include "boost/filesystem.hpp"  
#include "boost/regex.hpp"  
#include <iostream>  
   
using namespace boost::filesystem;  
   
int main()  
{  
  path current_dir(".");  
  // list all files starting with a  
  boost::regex pattern("a.*");  
  for (directory_iterator iter(current_dir), end;  
       iter != end;  
       ++iter)  
  {  
    boost::smatch match;  
    std::string fn = iter->path().filename().string(); // must make local variable  
    if (boost::regex_match( fn, match, pattern))  
    {  
      std::cout << match[0] << "\n";  
    }  
  }  
}

**Library:**  [std](http://rosettacode.org/mw/index.php?title=Category:Std&action=edit&redlink=1 "Category:Std (page does not exist)") **version**  C++17

   
#include <filesystem>  
#include <iostream>  
   
namespace fs = std::filesystem;  
   
int main() {  
  fs::path current_dir(".");  
  // list all files containing an mp3 extension  
  for (auto &file : fs::directory_iterator(current_dir)) {  
    if (file.path().extension() == ".mp3")  
      std::cout << file.path().filename().string() << std::endl;  
  }  
}

## [Clojure](http://rosettacode.org/wiki/Category:Clojure "Category:Clojure")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=14 "Edit section: Clojure")]

Using Java 8's  [PathMatcher](https://docs.oracle.com/javase/8/docs/api/java/nio/file/FileSystem.html#getPathMatcher-java.lang.String-)  patterns.

(import java.nio.file.FileSystems)  
   
(defn match-files [f pattern]  
  (.matches (.getPathMatcher (FileSystems/getDefault) (str "glob:*" pattern)) (.toPath f)))  
   
(defn walk-directory [dir pattern]  
  (let [directory (clojure.java.io/file dir)]  
    (map #(.getPath %) (filter #(match-files % pattern) (.listFiles directory)))))  
 

## [ColdFusion](http://rosettacode.org/wiki/Category:ColdFusion "Category:ColdFusion")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=15 "Edit section: ColdFusion")]

This example display all files and directories directly under  **C:\temp**  that end with  _.html_

<cfdirectory action="list" directory="C:\temp" filter="*.html" name="dirListing">  
<cfoutput query="dirListing">  
  #dirListing.name# (#dirListing.type#)<[br](http://december.com/html/4/element/br.html)>  
</cfoutput>

## [Common Lisp](http://rosettacode.org/wiki/Category:Common_Lisp "Category:Common Lisp")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=16 "Edit section: Common Lisp")]

(defun walk-directory (directory pattern)  
  (directory (merge-pathnames pattern directory)))

Uses the filename pattern syntax provided by the CL implementation.

## [D](http://rosettacode.org/wiki/Category:D "Category:D")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=17 "Edit section: D")]

void main() {  
    import std.stdio, std.file;  
   
    dirEntries(".", "*.*", SpanMode.shallow).writeln;  
}

## [DCL](http://rosettacode.org/wiki/Category:DCL "Category:DCL")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=18 "Edit section: DCL")]

* matches any number of characters
& matches exactly any one character

$ loop:  
$  f = f$search( p1 )  
$  if f .eqs. "" then $ exit  
$  write sys$output f  
$  goto loop

Output:

$ @walk_a_directory *.*
USERS:[DAVID]A.A;1
USERS:[DAVID]B.B;1
USERS:[DAVID]GG.GG;1
USERS:[DAVID]WALK_A_DIRECTORY.COM;1
$ @walk_a_directory *.%
USERS:[DAVID]A.A;1
USERS:[DAVID]B.B;1
$ @walk_a_directory *a*.*
USERS:[DAVID]A.A;1
USERS:[DAVID]WALK_A_DIRECTORY.COM;1
$ 

## [E](http://rosettacode.org/wiki/Category:E "Category:E")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=19 "Edit section: E")]

[def](http://wiki.erights.org/wiki/def) walkDirectory(directory, pattern) {  
  [for](http://wiki.erights.org/wiki/for) name => file ? (name =~ rx`.*$pattern.*`) [in](http://wiki.erights.org/wiki/in) directory {  
    [println](http://wiki.erights.org/wiki/println)(name)  
  }  
}

Example:

? walkDirectory(<file:~>, "bash_")  
.bash_history  
.bash_profile  
.bash_profile~

## [Elena](http://rosettacode.org/wiki/Category:Elena "Category:Elena")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=20 "Edit section: Elena")]

ELENA 4.0:

import system'io;  
import system'routines;  
import extensions'routines;  
   
public program()  
{  
    var dir := Directory.assign("c:\MyDir");  
   
    dir.getFiles("a.*").forEach:printingLn;  
}

## [Elixir](http://rosettacode.org/wiki/Category:Elixir "Category:Elixir")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=21 "Edit section: Elixir")]

# current directory  
IO.inspect File.ls!  
   
dir = "/users/public"  
IO.inspect File.ls!(dir)

Output:

["check.exs", "e.bat", "foo", "input.txt", "test.beam", "test.exs", "test.txt"]
["Desktop", "desktop.ini", "Documents", "Downloads", "Favorites", "Libraries",
 "Music", "Pictures", "Recorded TV", "Videos"]

## [Emacs Lisp](http://rosettacode.org/wiki/Category:Emacs_Lisp "Category:Emacs Lisp")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=22 "Edit section: Emacs Lisp")]

`directory-files`  gives filenames in a given directory, optionally restricted to those matching a regexp.

(directory-files "/some/dir/name"  
                 nil        ;; just the filenames, not full paths  
                 "\\.c\\'"  ;; regexp  
                 t)         ;; don't sort the filenames  
=>  
("foo.c" "bar.c" ...)

## [Erlang](http://rosettacode.org/wiki/Category:Erlang "Category:Erlang")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=23 "Edit section: Erlang")]

Use builtin function filelib:fold_files/5

Output:

8> filelib:fold_files( "/tmp", ".*", false, fun(File, Acc) -> [File|Acc] end, []).  
["/tmp/.X0-lock","/tmp/.cron-check-4000-was-here",
 "/tmp/kerneloops.XyN0SP","/tmp/npicagwD7tf"]
9> filelib:fold_files( "/tmp", "k.*P", false, fun(File, Acc) -> [File|Acc] end, []).
["/tmp/kerneloops.XyN0SP"]

## [Euphoria](http://rosettacode.org/wiki/Category:Euphoria "Category:Euphoria")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=24 "Edit section: Euphoria")]

include file.e  
   
procedure show(sequence pattern)  
    sequence f  
    f = dir(pattern)  
    for i = 1 to length(f) do  
        puts(1,f[i][D_NAME])  
        puts(1,'\n')  
    end for  
end procedure  
   
show("*.*")

## [F#](http://rosettacode.org/wiki/Category:F_Sharp "Category:F Sharp")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=25 "Edit section: F#")]

System.IO.Directory.GetFiles("c:\\temp", "*.xml")  
|> [Array](http://research.microsoft.com/en-us/um/cambridge/projects/fsharp/manual/namespaces.html).iter (printfn "%s")

## [Factor](http://rosettacode.org/wiki/Category:Factor "Category:Factor")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=26 "Edit section: Factor")]

Using unix globs. Also see the "directory." in basis/tools/files.factor.

USING: globs io io.directories kernel regexp sequences ;  
IN: walk-directory-non-recursively  
   
: print-files ( path pattern -- )  
    [ directory-files ] [ <glob> ] bi* [ matches? ] curry filter  
    [ print ] each ;

Ex:

   ( scratchpad ) "." "*.txt" print-files
   license.txt

## [Forth](http://rosettacode.org/wiki/Category:Forth "Category:Forth")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=27 "Edit section: Forth")]

**Works with**:  [gforth](http://rosettacode.org/wiki/Gforth "Gforth")  version 0.6.2

Gforth's directory walking functions are tied to the POSIX  _dirent_  functions, used by the C langauge entry above. Forth doesn't have regex support, so a simple filter function is used instead.

defer ls-filter ( name len -- ? )  
: ls-all  2drop true ;  
: ls-visible  drop c@ [char] . <> ;  
   
: ls ( dir len -- )  
  open-dir throw  ( dirid )  
  begin  
    dup pad 256 rot read-dir throw  
  while  
    pad over ls-filter if  
      cr pad swap type  
    else drop then  
  repeat  
  drop close-dir throw ;  
   
\ only show C language source and header files (*.c *.h)  
: c-file? ( str len -- ? )  
  dup 3 < if 2drop false exit then  
  + 1- dup c@  
   dup [char] c <> swap [char] h <> and if drop false exit then  
  1- dup c@ [char] . <> if drop false exit then  
  drop true ;  
' c-file? is ls-filter  
   
s" ." ls

## [Gambas](http://rosettacode.org/wiki/Category:Gambas "Category:Gambas")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=28 "Edit section: Gambas")]

**[Click this link to run this code](https://gambas-playground.proko.eu/?gist=c5fde952fecd1d7052101b1e2287f2ff)**

[Public](http://gambasdoc.org/help/lang/public) [Sub](http://gambasdoc.org/help/lang/sub) Main()  
[Dim](http://gambasdoc.org/help/lang/dim) sTemp [As](http://gambasdoc.org/help/lang/as) [String](http://gambasdoc.org/help/lang/type/string)  
   
[For](http://gambasdoc.org/help/lang/for) [Each](http://gambasdoc.org/help/lang/each) sTemp [In](http://gambasdoc.org/help/lang/in) [Dir](http://gambasdoc.org/help/lang/dir)("/etc", "*.d")  
  [Print](http://gambasdoc.org/help/lang/print) sTemp  
[Next](http://gambasdoc.org/help/lang/next)  
   
[End](http://gambasdoc.org/help/lang/end)

Output:

profile.d
rc1.d
rc4.d
rcS.d
binfmt.d
init.d
rc5.d
rc2.d

## [Go](http://rosettacode.org/wiki/Category:Go "Category:Go")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=29 "Edit section: Go")]

package main  
   
import (  
    "fmt"  
    "path/filepath"  
)  
   
func main() {  
    fmt.Println(filepath.Glob("*.go"))  
}

## [Groovy](http://rosettacode.org/wiki/Category:Groovy "Category:Groovy")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=30 "Edit section: Groovy")]

// *** print *.txt files in current directory  
 [new](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20new) [File](https://www.google.de/search?as_q=File&num=100&hl=en&as_occt=url&as_sitesearch=java.sun.com%2Fj2se%2F1%2E5%2E0%2Fdocs%2Fapi%2F)('.').eachFileMatch(~/.*\.txt/) {  
   [println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) it  
 }  
   
 // *** print *.txt files in /foo/bar  
 [new](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20new) [File](https://www.google.de/search?as_q=File&num=100&hl=en&as_occt=url&as_sitesearch=java.sun.com%2Fj2se%2F1%2E5%2E0%2Fdocs%2Fapi%2F)('/foo/bar').eachFileMatch(~/.*\.txt/) {  
   [println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) it  
 }

## [Haskell](http://rosettacode.org/wiki/Category:Haskell "Category:Haskell")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=31 "Edit section: Haskell")]

**Works with**:  [GHCi](http://rosettacode.org/wiki/GHC "GHC")  version 6.6

In this example, the pattern is a POSIX extended regular expression.

import System.Directory  
import Text.Regex  
import Data.[Maybe](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:Maybe)  
   
walk :: FilePath -> [String](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:String) -> [IO](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:IO) ()  
walk dir pattern = do  
    filenames <- getDirectoryContents dir  
    [mapM_](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:mapM_) [putStrLn](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:putStrLn) $ [filter](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:filter) (isJust.(matchRegex $ mkRegex pattern)) filenames  
   
main = walk "." ".\\.hs$"

## [HicEst](http://rosettacode.org/wiki/Category:HicEst "Category:HicEst")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=32 "Edit section: HicEst")]

More on  [SYSTEM](http://www.hicest.com/SYSTEM.htm),  [OPEN](http://www.hicest.com/OPEN.htm),  [INDEX](http://www.hicest.com/indexfnc.htm)

CHARACTER dirtxt='dir.txt', filename*80  
   
SYSTEM(DIR='*.*', FIle=dirtxt) ! "file names", length, attrib, Created, LastWrite, LastAccess  
OPEN(FIle=dirtxt, Format='"",', LENgth=files) ! parses column 1 ("file names")  
   
DO nr = 1, files  
  filename = dirtxt(nr,1) ! reads dirtxt row = nr, column = 1 to filename  
  ! write file names with extensions "txt", or "hic", or "jpg" (case insensitive) using RegEx option =128:  
  IF( INDEX(filename, "\.txt|\.hic|\.jpg", 128) ) WRITE() filename   
ENDDO

## [Icon](http://rosettacode.org/wiki/Category:Icon "Category:Icon")  and  [Unicon](http://rosettacode.org/wiki/Category:Unicon "Category:Unicon")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=33 "Edit section: Icon and Unicon")]

This uses Unicon extensions for  _stat_  and to read directories. Icon can uses  _system_  to accomplish the same objective.

procedure main()  
every write(getdirs(".","icn"))  # writes out all directories from the current directory down  
end  
   
procedure getdirs(s,pat)  #: return a list of directories beneath the directory 's'  
local d,f  
   
if ( stat(s).mode ? ="d" ) & ( d := open(s) ) then {  
      while f := read(d) do   
         if find(pat,f) then   
            suspend f  
      close(d)  
      }  
end

## [IDL](http://rosettacode.org/wiki/Category:IDL "Category:IDL")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=34 "Edit section: IDL")]

f = file_search('*.txt', count=cc)  
if cc gt 0 then print,f

(IDL is an array language - very few things are ever done in 'loops'.)

## [J](http://rosettacode.org/wiki/Category:J "Category:J")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=35 "Edit section: J")]

require 'dir'  
0 dir '*.png'  
0 dir '/mydir/*.txt'

The verb  dir  supports a number of reporting options determined by its left argument. A left argument of  0  reports just the file names.

## [Java](http://rosettacode.org/wiki/Category:Java "Category:Java")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=36 "Edit section: Java")]

[File](https://www.google.com/search?hl=en&q=allinurl%3Afile+java.sun.com&btnI=I%27m%20Feeling%20Lucky) dir = new [File](https://www.google.com/search?hl=en&q=allinurl%3Afile+java.sun.com&btnI=I%27m%20Feeling%20Lucky)("/foo/bar");  
   
[String](https://www.google.com/search?hl=en&q=allinurl%3Astring+java.sun.com&btnI=I%27m%20Feeling%20Lucky)[] contents = dir.list();  
for ([String](https://www.google.com/search?hl=en&q=allinurl%3Astring+java.sun.com&btnI=I%27m%20Feeling%20Lucky) file : contents)  
    if (file.endsWith(".mp3"))  
        [System](https://www.google.com/search?hl=en&q=allinurl%3Asystem+java.sun.com&btnI=I%27m%20Feeling%20Lucky).out.println(file);

## [JavaScript](http://rosettacode.org/wiki/Category:JavaScript "Category:JavaScript")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=37 "Edit section: JavaScript")]

**Works with**:  [JScript](http://rosettacode.org/wiki/JScript "JScript")

var fso = new ActiveXObject("Scripting.FileSystemObject");  
var dir = fso.GetFolder('test_folder');  
   
function walkDirectory(dir, re_pattern) {  
    WScript.Echo("Files in " + dir.name + " matching '" + re_pattern +"':");  
    walkDirectoryFilter(dir.Files, re_pattern);  
   
    WScript.Echo("Folders in " + dir.name + " matching '" + re_pattern +"':");  
    walkDirectoryFilter(dir.Subfolders, re_pattern);  
}  
   
function walkDirectoryFilter(items, re_pattern) {  
    var e = new Enumerator(items);  
    while (! e.atEnd()) {  
        var item = e.item();  
        if (item.name.match(re_pattern))  
            WScript.Echo(item.name);  
        e.moveNext();  
    }  
}  
   
walkDirectory(dir, '\\.txt$');

## [Julia](http://rosettacode.org/wiki/Category:Julia "Category:Julia")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=38 "Edit section: Julia")]

**Works with**:  [Julia](http://rosettacode.org/wiki/Julia "Julia")  version 0.6

for filename in readdir("/foo/bar")  
    if endswith(filename, ".mp3")  
        print(filename)  
    end  
end

## [Kotlin](http://rosettacode.org/wiki/Category:Kotlin "Category:Kotlin")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=39 "Edit section: Kotlin")]

// version 1.1.2  
   
[import](https://scala-lang.org/) java.io.File  
   
fun walkDirectory(dirPath: String, pattern: Regex): List<String> {  
    [val](https://scala-lang.org/) d = File(dirPath)  
    require(d.exists() && d.isDirectory())  
    [return](https://scala-lang.org/) d.list().filter { it.matches(pattern) }  
}  
   
fun main(args: Array<String>) {  
    [val](https://scala-lang.org/) r = Regex("""^a.*\.h$""")  // get all C header files beginning with 'a'  
    [val](https://scala-lang.org/) files = walkDirectory("/usr/include", r)  
    [for](https://scala-lang.org/) (file in files) println(file)  
}

Sample output (Ubuntu v14.04):

Output:

argp.h
alloca.h
ar.h
aliases.h
autosprintf.h
aio.h
assert.h
argz.h

## [Lasso](http://rosettacode.org/wiki/Category:Lasso "Category:Lasso")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=40 "Edit section: Lasso")]

local(matchingfilenames = array)  
   
dir('.') -> foreach => {#1 >> 'string' ? #matchingfilenames -> insert(#1)}  
   
#matchingfilenames

-> array(mystrings.html, a_string_file.txt)

## [Lingo](http://rosettacode.org/wiki/Category:Lingo "Category:Lingo")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=41 "Edit section: Lingo")]

-- Usage: printFiles("C:\scripts", ".ls")  
on printFiles (dir, fileType)  
  i = 1  
  sub = fileType.length -1  
  repeat while TRUE  
    fn = getNthFileNameInFolder(dir, i)   
    if fn = EMPTY then exit repeat  
    i = i+1  
    if fn.length<fileType.length then next repeat  
    if fn.char[fn.length-sub..fn.length]=fileType then put fn  
  end repeat  
end

## [LiveCode](http://rosettacode.org/wiki/Category:LiveCode "Category:LiveCode")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=42 "Edit section: LiveCode")]

set the defaultFolder to the documents folder  -- the documents folder is a "specialFolderPath"  
put the files into docfiles  
filter docfiles with "*.txt"  
put docfiles

## [Lua](http://rosettacode.org/wiki/Category:Lua "Category:Lua")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=43 "Edit section: Lua")]

Lua itself is extremely spartanic as it is meant for embedding. Reading out a directory is not something that a minimal standard C library can do, and so minimal Lua without native extension libraries can't do it either. But lfs (LuaFileSystem) is about as standard an extension as it gets, so we use that.

require "lfs"  
directorypath = "." -- current working directory  
for filename in lfs.dir(directorypath) do  
    if filename:match("%.lua$") then -- "%." is an escaped ".", "$" is end of string  
        print(filename)  
    end  
end

Although Lua is spartanic, it still provides functions such as os.execute([command]) and io.popen(prog [, mode]). Below an example for Windows users having io.popen at their disposal. Mind you, it may pop-up a command window.

-- Gets the output of given program as string  
-- Note that io.popen is not available on all platforms  
local function getOutput(prog)  
    local file = assert(io.popen(prog, "r"))  
    local output = assert(file:read("*a"))  
    file:close()  
    return output  
end  
   
-- Iterates files in given directory  
local function files(directory, recursively)  
    -- Use windows" dir command  
    local directory = directory:gsub("/", "\\")  
    local filenames = getOutput(string.format("dir %s %s/B/A:A", directory, recursively and '/S' or ''))  
   
    -- Function to be called in "for filename in files(directory)"  
    return coroutine.wrap(function()  
        for filename in filenames:gmatch("([^\r\n]+)") do  
            coroutine.yield(filename)  
        end      
    end)  
end  
   
-- Walk "C:/Windows" looking for executables  
local directory = "C:/Windows"  
local pattern = ".*%.exe$" -- for finding executables  
for filename in files(directory) do  
    if filename:match(pattern) then  
        print(filename)  
    end  
end

## [M2000 Interpreter](http://rosettacode.org/wiki/Category:M2000_Interpreter "Category:M2000 Interpreter")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=44 "Edit section: M2000 Interpreter")]

Console has a popup list called Menu, which we can fill using Files statements. Files statement get some symbols before first argument for sorting and to not export to console but to menu list. So we can use MenuItems to find how many items return, and we can walk menu array to get the names (from 1 to MenuItems).

Files statement get as first argument a pattern or a list of file extensions "txt|bmp" return these two kind of files. There is a second optional parameter which examine all files founded from first filter for included letters. We can add using | as seperator, a list of strings included in same line. Files examine all files, opened one by one, using an automatic way to find what kind of text file is, an Ansi, a Utf8, a Utf-16LE, or a Utf-16BE. Also automatic find the line breaks. All files converted at open as utf-16LE and then searched. For Ansi files, Locale used to make the right conversion.

   
Module Show_Files_Standard {  
      \\ we get more (include hidden too)  
      Module InnerWay (folder_path$, pattern$){  
            olddir$=dir$  
            dir folder_path$  
            \\ clear menu list  
            Menu  
            \\ + place export to menu, without showing  
            \\ ! sort to name  
            files ! + pattern$  
            If MenuItems>0 then {  
                  For i=1 to MenuItems {  
                        Print Menu$(i)+".exe"  
                  }  
            }  
            dir olddir$  
      }  
      InnerWay "C:\Windows","*.exe"  
}  
Show_Files_Standard  
 

Like VbScript using external help, from a COM object.

We use an enumerator to iterate all file names and checked using like operator "~",and then we push them to end of stack (Data push to end), so we get first the first entered (we use stack here as a FIFO, using a New stack for temporary use), and we remove at the end all items and place them in an array. This array return from get_file$() and we make a second iterator for array, to get each end display it. The second iterator is not a com enumerator, but another type of object included in this interpreter. This iterator can get start and end position, defining a range and a direction too.

EnumFile is an object in an object. In expression we get the inner object. In While {} we get the outer object, and iterate or not (depends of state), so the inner object change. Because we get the first object at creation time, the first time when While structure found this object skips iteration.

Stack New {} make a block of a fresh stack of values, and at the exit attach the old stack (which for this block detached from execute object at the begin of block).

   
Module Show_Files {  
      Function  get_files$ (folder_path$) {  
            \\ we get second argument using letter$ which pop from stack  
            pattern$=lcase$(Letter$)  
            Declare  objfso "Scripting.FileSystemObject"  
            Method objfso, "GetFolder", folder_path$ as fc  
            With fc, "files" set files  
            \\ from revision 13 - version 9.4  
            With files, -4& as EnumFile  
            With EnumFile, "Name" as name$  
            Dim empty$()  
            =empty$()  
            Stack New {  
                  While EnumFile {  
                        If lcase$(name$) ~ pattern$ Then Data name$  
                  }  
                  \\ get stack values and fill an array  
                  =Array$([])  
            }  
      }  
      Dim Name$()  
      Name$()=get_files$("C:\Windows","*.exe")  
      m=each(Name$())  
      While m {  
            Print Array$(m)  
      }  
}  
Show_Files  
 

## [Mathematica](http://rosettacode.org/wiki/Category:Mathematica "Category:Mathematica")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=45 "Edit section: Mathematica")]

The built-in function  `FileNames`  does exactly this:

`FileNames[]`  lists all files in the current working directory.

`FileNames[form]`  lists all files in the current working directory whose names match the string pattern form.

`FileNames[{form1,form2,...}]`  lists all files whose names match any of the form_i.

`FileNames[forms,{dir1,dir2,...}]`  lists files with names matching forms in any of the directories dir_i.

`FileNames[forms,dirs,n]`  includes files that are in subdirectories up to n levels down.

Examples (find all files in current directory, find all png files in root directory):

FileNames["*"]  
FileNames["*.png", $RootDirectory]

the result can be printed with Print /@ FileNames[....].

## [MAXScript](http://rosettacode.org/wiki/Category:MAXScript "Category:MAXScript")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=46 "Edit section: MAXScript")]

getFiles "C:\\*.txt"

## [Nemerle](http://rosettacode.org/wiki/Category:Nemerle "Category:Nemerle")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=47 "Edit section: Nemerle")]

using System.Console;  
using System.IO;  
   
module DirWalk  
{  
    Main() : void  
    {  
        def files = Directory.GetFiles(@"C:\MyDir");                  // retrieves only files  
        def files_subs = Directory.GetFileSystemEntries(@"C:\MyDir"); // also retrieves (but does not enter) sub-directories  
                                                                      // (like ls command)  
        foreach (file in files) WriteLine(file);  
    }  
}

## [NetRexx](http://rosettacode.org/wiki/Category:NetRexx "Category:NetRexx")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=48 "Edit section: NetRexx")]

/* NetRexx */  
options replace format comments java crossref symbols nobinary  
   
import java.util.List  
   
runSample(arg)  
return  
   
-- ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~  
method getFileNames(dirname, pattern) public static returns List  
  dir = File(dirname)  
  contents = dir.list()  
  fileNames = ArrayList()  
  loop fname over contents  
    if fname.matches(pattern) then do  
      fileNames.add(fname)  
      end  
    end fname  
  Collections.sort(fileNames)  
  return fileNames  
   
-- ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~  
method runSample(arg) private static  
  parse arg dirname pattern  
  if dirname = '' then dirname = System.getProperty('user.dir')  
  if pattern = '' then pattern = '^RW.*\\.nrx$'  
   
  fileNames = getFileNames(dirname, pattern)  
  say 'Search of' dirname 'for files matching pattern "'pattern'" found' fileNames.size() 'files.'  
  loop fn = 0 while fn < fileNames.size()  
    say (fn + 1).right(5)':' fileNames.get(fn)  
    end fn  
   
  return  
 

Output:

Search of /Users/projects/RosettaCode/netrexx for files matching pattern "^RW.*\.nrx$" found 5 files.
    1: RWalkDir_Iter.nrx
    2: RWebScraping.nrx
    3: RWindowCreate.nrx
    4: RWriteFloatArray.nrx
    5: RWriteName3D01.nrx

## [Nim](http://rosettacode.org/wiki/Category:Nim "Category:Nim")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=49 "Edit section: Nim")]

import os  
   
for file in walkFiles "/foo/bar/*.mp3":  
  echo file

## [Objeck](http://rosettacode.org/wiki/Category:Objeck "Category:Objeck")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=50 "Edit section: Objeck")]

use IO;  
   
bundle Default {  
  class Test {  
    function : Main(args : System.String[]) ~ Nil {  
       dir := Directory->List("/src/code");  
       for(i := 0; i < dir->Size(); i += 1;) {  
         if(dir[i]->EndsWith(".obs")) {  
           dir[i]->PrintLine();  
        };  
      };  
    }  
  }  
}

## [Objective-C](http://rosettacode.org/wiki/Category:Objective-C "Category:Objective-C")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=51 "Edit section: Objective-C")]

[NSString](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSString_Class/) *dir = @"/foo/bar";  
   
// Pre-OS X 10.5  
[NSArray](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSArray_Class/) *contents = [[[NSFileManager](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSFileManager_Class/) defaultManager] directoryContentsAtPath:dir];  
// OS X 10.5+  
[NSArray](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSArray_Class/) *contents = [[[NSFileManager](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSFileManager_Class/) defaultManager] contentsOfDirectoryAtPath:dir error:NULL];  
   
for ([NSString](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSString_Class/) *file in contents)  
  if ([[file pathExtension] isEqualToString:@"mp3"])  
    NSLog(@"%@", file);

## [OCaml](http://rosettacode.org/wiki/Category:OCaml "Category:OCaml")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=52 "Edit section: OCaml")]

#load "str.cma"  
let contents = [Array](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Array.html).to_list ([Sys](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Sys.html).readdir ".") in  
let select pat str = [Str](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Str.html).string_match ([Str](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Str.html).regexp pat) str 0 in  
[List](http://caml.inria.fr/pub/docs/manual-ocaml/libref/List.html).filter (select ".*\\.jpg") contents

## [Oz](http://rosettacode.org/wiki/Category:Oz "Category:Oz")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=53 "Edit section: Oz")]

declare  
  [Path] = {Module.link ['x-oz://system/os/Path.ozf']}  
  [Regex] = {Module.link ['x-oz://contrib/regex']}  
   
  Files = {Filter {Path.readdir "."} Path.isFile}  
  Pattern = ".*\\.oz$"  
  MatchingFiles = {Filter Files fun {$ File} {Regex.search Pattern File} \= false end}  
in  
  {ForAll MatchingFiles System.showInfo}

## [Pascal](http://rosettacode.org/wiki/Category:Pascal "Category:Pascal")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=54 "Edit section: Pascal")]

**Works with**:  [Free Pascal](http://rosettacode.org/wiki/Free_Pascal "Free Pascal")

{$H+}  
   
program Walk;  
   
uses SysUtils;  
   
var Res: TSearchRec;  
    Pattern, Path, Name: String;  
    FileAttr: LongInt;  
    Attr: Integer;  
   
begin  
   Write('File pattern: ');  
   ReadLn(Pattern);            { For example .\*.pas }  
   
   Attr := faAnyFile;  
   if FindFirst(Pattern, Attr, Res) = 0 then  
   begin  
      Path := ExtractFileDir(Pattern);  
      repeat  
         Name := ConcatPaths([Path, Res.Name]);  
         FileAttr := FileGetAttr(Name);  
         if FileAttr and faDirectory = 0 then  
         begin  
            { Do something with file name }  
            WriteLn(Name);  
         end  
      until FindNext(Res) <> 0;  
   end;  
   FindClose(Res);  
end.

## [Perl](http://rosettacode.org/wiki/Category:Perl "Category:Perl")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=55 "Edit section: Perl")]

use 5.010;  
[opendir](https://perldoc.perl.org/functions/opendir.html) my $dh, '/home/foo/bar';  
say for [grep](https://perldoc.perl.org/functions/grep.html) { /php$/ } [readdir](https://perldoc.perl.org/functions/readdir.html) $dh;  
[closedir](https://perldoc.perl.org/functions/closedir.html) $dh;

Or using globbing, with the  `<>`  operator,

use 5.010; say while </home/foo/bar/*.php>;

Or the same with the builtin  `glob()`  function,

my @filenames = [glob](https://perldoc.perl.org/functions/glob.html)('/home/foo/bar/*.php');

The  `glob()`  function takes any expression for its pattern, whereas  `<>`  is only for a literal.

my $pattern = '*.c';  
my @filenames = [glob](https://perldoc.perl.org/functions/glob.html)($pattern);

## [Perl 6](http://rosettacode.org/wiki/Category:Perl_6 "Category:Perl 6")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=56 "Edit section: Perl 6")]

The  `dir`  function takes the directory to traverse, and optionally a named parameter  `test`, which is  [smart-matched](https://docs.perl6.org/routine/$TILDE$TILDE)  against the basename of each file (so for example we can use a regex):

.say for dir ".", :test(/foo/);

## [Phix](http://rosettacode.org/wiki/Category:Phix "Category:Phix")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=57 "Edit section: Phix")]

The dir function accepts a DOS pattern, with some minor variations (eg "*" gets all files with no extension).

puts(1,join(columnize(dir("*.txt"))[D_NAME],"\n"))

Output:

copyright.txt
e-1millon.txt
ildump.txt
output.txt
readme.txt
_TODO.TXT 

## [PHP](http://rosettacode.org/wiki/Category:PHP "Category:PHP")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=58 "Edit section: PHP")]

**Works with**:  [PHP](http://rosettacode.org/wiki/PHP "PHP")  version 5.2.0

$pattern = 'php';  
$dh = [opendir](http://www.php.net/opendir)('c:/foo/bar'); // Or '/home/foo/bar' for Linux  
while (false !== ($file = [readdir](http://www.php.net/readdir)($dh)))  
{  
    if ($file != '.' and $file != '..')  
    {  
        if ([preg_match](http://www.php.net/preg_match)("/$pattern/", $file))  
        {  
            echo "$file matches $pattern\n";  
        }  
    }  
}  
[closedir](http://www.php.net/closedir)($dh);

Or:

$pattern = 'php';  
foreach ([scandir](http://www.php.net/scandir)('/home/foo/bar') as $file)  
{  
    if ($file != '.' and $file != '..')  
    {  
        if ([preg_match](http://www.php.net/preg_match)("/$pattern/", $file))  
        {  
            echo "$file matches $pattern\n";  
        }  
    }  
}

**Works with**:  [PHP](http://rosettacode.org/wiki/PHP "PHP")  version 4 >= 4.3.0 or 5

foreach ([glob](http://www.php.net/glob)('/home/foo/bar/*.php') as $file){  
    echo "$file\n";  
}

## [PicoLisp](http://rosettacode.org/wiki/Category:PicoLisp "Category:PicoLisp")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=59 "Edit section: PicoLisp")]

(for F (dir "@src/")                         # Iterate directory  
   (when (match '`(chop "s@.c") (chop F))    # Matches 's*.c'?  
      (println F) ) )                        # Yes: Print it

Output:

"start.c"
"ssl.c"
"subr.c"
"sym.c"
...

## [Pike](http://rosettacode.org/wiki/Category:Pike "Category:Pike")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=60 "Edit section: Pike")]

array(string) files = get_dir("/home/foo/bar");  
foreach(files, string file)  
    write(file + "\n");

## [Pop11](http://rosettacode.org/wiki/Category:Pop11 "Category:Pop11")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=61 "Edit section: Pop11")]

Built-in procedure sys_file_match searches directories (or directory trees) using shell-like patterns:

lvars repp, fil;  
;;; create path repeater  
sys_file_match('*.p', '', false, 0) -> repp;  
;;; iterate over files  
while (repp() ->> fil) /= termin do  
     ;;; print the file  
     printf(fil, '%s\n');  
endwhile;

## [PowerShell](http://rosettacode.org/wiki/Category:PowerShell "Category:PowerShell")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=62 "Edit section: PowerShell")]

Since PowerShell is also a shell it should come as no surprise that this task is very simple. Listing the names of all text files, or the names of all files, starting with "f":

Get-ChildItem *.txt -Name  
Get-ChildItem f* -Name

The  `-Name`  parameter tells the  `Get-ChildItem`  to return only the file names as string, otherwise a complete  `FileInfo`  or  `DirectoryInfo`  object would be returned, containing much more information than only the file name.

More complex matching can be accomplished by filtering the complete list of files using the  `Where-Object`  cmdlet. The following will output all file names that contain at least one vowel:

Get-ChildItem -Name | Where-Object { $_ -match '[aeiou]' }

## [PureBasic](http://rosettacode.org/wiki/Category:PureBasic "Category:PureBasic")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=63 "Edit section: PureBasic")]

The match is made using DOS wildcards. It could easily be modified to match based on a regular expression if desired (i.e. using the PCRE library).

Procedure walkDirectory(directory.s = "", pattern.s = "")  
  Protected directoryID  
   
  directoryID = ExamineDirectory(#PB_Any,directory,pattern)  
  If directoryID  
    While NextDirectoryEntry(directoryID)  
      PrintN(DirectoryEntryName(directoryID))  
    Wend  
    FinishDirectory(directoryID)  
  EndIf   
EndProcedure  
   
If OpenConsole()  
  walkDirectory()    
   
  Print(#CRLF$ + #CRLF$ + "Press ENTER to exit")  
  Input()  
  CloseConsole()  
EndIf

## [Python](http://rosettacode.org/wiki/Category:Python "Category:Python")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=64 "Edit section: Python")]

The  [glob](https://python.org/doc/lib/module-glob.html)  library included with Python lists files matching shell-like patterns:

import glob  
for filename in glob.glob('/foo/bar/*.mp3'):  
    print(filename)

Or manually:

import os  
for filename in os.listdir('/foo/bar'):  
    if filename.endswith('.mp3'):  
        print(filename)

## [R](http://rosettacode.org/wiki/Category:R "Category:R")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=65 "Edit section: R")]

dir("/foo/bar", "mp3")

## [Racket](http://rosettacode.org/wiki/Category:Racket "Category:Racket")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=66 "Edit section: Racket")]

   
-> (for ([f (directory-list "/tmp")] #:when (regexp-match? "\\.rkt$" f))  
     (displayln f))  
... *.rkt files ...  
 

## [Rascal](http://rosettacode.org/wiki/Category:Rascal "Category:Rascal")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=67 "Edit section: Rascal")]

import IO;  
public void Walk(loc a, str pattern){  
	for (entry <- listEntries(a))  
		endsWith(entry, pattern) ? println(entry);  
}

## [Raven](http://rosettacode.org/wiki/Category:Raven "Category:Raven")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=68 "Edit section: Raven")]

'dir://.' open each as item  
    item m/\.txt$/ if "%(item)s\n" print

## [REXX](http://rosettacode.org/wiki/Category:REXX "Category:REXX")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=69 "Edit section: REXX")]

**Works with**:  [Regina](http://rosettacode.org/wiki/Regina "Regina")

The following program was tested in a DOS window under Windows/XP and should work for all Microsoft Windows.

/*REXX program shows files in directory tree that match a given criteria*/  
parse arg xdir;  if xdir='' then xdir='\'        /*Any DIR? Use default.*/  
@.=0                                   /*default in case ADDRESS fails. */  
trace off                              /*suppress REXX err msg for fails*/  
address system 'DIR' xdir '/b /s' with output stem @.   /*issue DIR cmd.*/  
if rc\==0  then do                                  /*an error happened?*/  
                say '***error!*** from DIR' xDIR    /*indicate que pasa.*/  
                say 'return code='  rc              /*show the Ret Code.*/  
                exit rc                             /*exit with the  RC.*/  
                end                                 /* [↑]  bad address.*/  
#=@.rc                                              /*number of entries.*/  
if #==0  then #='   no   '                          /*use a word, ¬zero.*/  
say center('directory ' xdir " has "    #     ' matching entries.',79,'─')  
   
       do j=1  for #;  say @.j;  end   /*show files that met criteria.  */  
   
exit @.0+rc                            /*stick a fork in it, we're done.*/

  

## [Ring](http://rosettacode.org/wiki/Category:Ring "Category:Ring")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=70 "Edit section: Ring")]

   
   
###---------------------------------------  
### Directory Tree Walk  
### Look for FileType for Music and Video  
   
fileType = [".avi", ".mp4", ".mpg", ".mkv", ".mp3", ".wmv" ]  
   
dirList   = []  
musicList = []  
   
###---------------------------------------  
### Main  
   
    ###-----------------------------------  
    ### Start at this directory  
   
    searchVideoMusic("C:\Users\Umberto\")  
   
    see nl +"Number of Music and Videos files: " +len(musicList) +nl +nl  
    see musicList  
    See nl +"Finished" +nl  
   
###=======================================  
### Search for Video and Music files  
   
Func searchVideoMusic(startDir)  
   
    ChDir(startDir + "Music")     ### <<<== add Music subpath C:\Users\Umberto\Music  
    listDir( CurrentDir() )  
   
    ChDir(startDir + "Videos")    ### <<<== add Videos subpath C:\Users\Umberto\Videos  
    listDir( CurrentDir() )  
   
    for searchDir in dirList      ### Search Directory List for Music and Videos files  
        listDir(searchDir)  
    next  
   
   
###==============================  
### Find Files in Directory  
   
Func listDir(dirName)  
   
    ChDir(dirName)  
    Try  
        ###-------------------------------------  
        ### Get SubDirectories  
   
        myListSub = Dir( CurrentDir() )  
    Catch  
        ###-------------------------------------  
        ### Error, Couldn't open the directory  
   
        See "ListDir Catch! " + CurrentDir() +" --- "+ cCatchError +nl  
        return  
    Done  
   
    for x in myListSub  
        if x[2]  
            thisDir = x[1]  
   
            if thisDir[1] = "."  
                ### Do Nothing. Ignore dot.name  
   
            else  
                see nl +"Dir: " + CurrentDir() +"\"+ thisDir + nl  
   
                ###----------------------------------------  
                ###  Directory Walk add to directory list  
   
                Add( dirList, (CurrentDir() +"\"+  thisDir))  
            ok  
        else  
            thisFile = x[1]  
   
            ###-------------------------------  
            ### Add Music or Video file type  
   
            for thisType in fileType  
                if ( substr(thisFile, thisType) )             ### <<<== Type of File from List  
                     see "         File: " + thisFile + nl  
                     Add(musicList, (CurrentDir() +"\"+  thisFile))  
                ok  
            next  
        ok  
    next  
return  
   
###===============================================  
   
 

OUTPUT:

Dir: C:\Users\Umberto\Music\Free YouTube Downloader
         File: stock.mp3
         File: big_buck_bunny.mp4
         File: BowieNikolaTesla'ThePrestige'.mpg
         File: BowieTesla'The Prestige'.wmv
         File: Candyman.mp4

Dir: C:\Users\Umberto\Videos\Captures
         File: drop.avi

Dir: C:\Users\Umberto\Videos\Free YouTube Downloader
         File: GaryUSBondsQuarterToThree.avi

Dir: C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]
         File: Joe.Versus.The.Volcano[1990].avi
         File: SampleTheMythSanWa2005.mkv
         File: stock.mp3

Dir: C:\Users\Umberto\Videos\The Prestige (2006)
         File: BowieNikola'The Prestige'.mp4
         File: BowieTeslaThe PrestigeConverted.mpg
         File: 027_3xplanet_MDYD-895.avi
         File: 3.mpg
         File: HitomiTanaka[MIDE-219].mp4
         File: MDYD-868.wmv
         File: MIDE-253.mp4
         File: MIDE_280.mp4
         File: PPPD-432.avi
         File: The.Prestige.2006.mkv

Number of Music and Videos files: 20

C:\Users\Umberto\Music\stock.mp3
C:\Users\Umberto\Videos\big_buck_bunny.mp4
C:\Users\Umberto\Videos\BowieNikolaTesla'ThePrestige'.mpg
C:\Users\Umberto\Videos\BowieTesla'The Prestige'.wmv
C:\Users\Umberto\Videos\Candyman.mp4
C:\Users\Umberto\Videos\drop.avi
C:\Users\Umberto\Videos\GaryUSBondsQuarterToThree.avi
C:\Users\Umberto\Videos\Joe.Versus.The.Volcano[1990].avi
C:\Users\Umberto\Videos\SampleTheMythSanWa2005.mkv
C:\Users\Umberto\Videos\stock.mp3
C:\Users\Umberto\Videos\Free YouTube Downloader\BowieNikola'The Prestige'.mp4
C:\Users\Umberto\Videos\Free YouTube Downloader\BowieTeslaThe PrestigeConverted.mpg
C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]\027_3xplanet_MDYD-895.avi
C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]\3.mpg
C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]\HitomiTanaka[MIDE-219].mp4
C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]\MDYD-868.wmv
C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]\MIDE-253.mp4
C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]\MIDE_280.mp4
C:\Users\Umberto\Videos\HitomiTanaka[MIDE-219]\PPPD-432.avi
C:\Users\Umberto\Videos\The Prestige (2006)\The.Prestige.2006.mkv

Finished

## [Ruby](http://rosettacode.org/wiki/Category:Ruby "Category:Ruby")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=71 "Edit section: Ruby")]

# Files under this directory:  
Dir.glob('*') { |file| puts file }  
   
# Files under path '/foo/bar':  
Dir.glob( File.join('/foo/bar', '*') ) { |file| puts file }  
   
# As a method  
def file_match(pattern=/\.txt/, path='.')  
  Dir[File.join(path,'*')].each do |file|  
    puts file if file =~ pattern  
  end  
end

## [Run BASIC](http://rosettacode.org/wiki/Category:Run_BASIC "Category:Run BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=72 "Edit section: Run BASIC")]

files #g, DefaultDir$ + "\*.jpg"   ' find all jpg files   
   
if #g HASANSWER() then  
	count = #g rowcount()	' get count of files  
	for i = 1 to count  
	if #g hasanswer() then	'retrieve info for next file  
		#g nextfile$()	'print name of file  
		print #g NAME$()  
	end if  
	next  
end if  
wait

FILE ACCESSOR methods

1.  handle HASANSWER() - Return non-zero if the file accessor has at least one resulting row.
2.  handle ROWCOUNT() - Return the number of rows returned.
3.  handle NEXTFILE$() - Advance to the next row and return a comma delimited string for the next file (name, size, date, time, directory flag).
4.  handle NEXTFILE$([delimExpr$]) - Like NEXTFILE$() but you get to specify the delimiter instead of a comma.
5.  handle NAME$() - Return the name of the current file row.
6.  handle SIZE() - Return the size of the current file row.
7.  handle DATE$() - Return a string containing a formatted date for the current file row.
8.  handle TIME$() - Return a string containing a formatted time for the current file row.
9.  handle ISDIR() - Return non-zero if the current file row represents a directory instead of a file.
10.  handle RESET() - Reset the file accessor back to the beginning so you can read through them again.
11.  handle DATEFORMAT(template$) - Set the date format using a "mmm dd, yyyy" style template$.
12.  handle TIMEFORMAT(template$) - Set the time format using a "hh:mm:ss" style template$.
13.  handle ISNULL() - Returns zero (or false)
14.  handle DEBUG$() - Returns the string "Files"

OUTPUT:

button.JPG
circuitbanner1.JPG
circuitbanner2.JPG
copy.jpg
homecomputerbanner1.JPG
mandelbrot.jpg

## [Rust](http://rosettacode.org/wiki/Category:Rust "Category:Rust")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=73 "Edit section: Rust")]

```rust
extern crate docopt;  
extern crate regex;  
extern crate rustc_serialize;  
   
use docopt::Docopt;  
use regex::Regex;  
   
const USAGE: &'static str = "  
Usage: rosetta <pattern>  
   
Walks the directory tree starting with the current working directory and  
print filenames matching <pattern>.  
";  
   
#[derive(Debug, RustcDecodable)]  
struct Args {  
    arg_pattern: String,  
}  
   
fn main() {  
    let args: Args = Docopt::new(USAGE)  
        .and_then(|d| d.decode())  
        .unwrap_or_else(|e| e.exit());  
   
    let re = Regex::new(&args.arg_pattern).unwrap();  
    let paths = std::fs::read_dir(".").unwrap();  
   
    for path in paths {  
        let path = path.unwrap().path();  
        let path = path.to_str().unwrap();  
   
        if re.is_match(path) {  
            println!("{}", path);  
        }  
    }  
}
```

## [Scala](http://rosettacode.org/wiki/Category:Scala "Category:Scala")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=74 "Edit section: Scala")]

[import](https://scala-lang.org/) java.io.File  
   
[val](https://scala-lang.org/) dir = [new](https://scala-lang.org/) File("/foo/bar").list()  
dir.filter(file => file.endsWith(".mp3")).foreach(println)

## [Seed7](http://rosettacode.org/wiki/Category:Seed7 "Category:Seed7")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=75 "Edit section: Seed7")]

$ include "seed7_05.s7i";  
  include "osfiles.s7i";  
   
const proc: main is func  
  local  
    var string: fileName is "";  
  begin  
    for fileName range readDir(".") do  
      if endsWith(fileName, ".sd7") then  
        writeln(fileName);  
      end if;  
    end for;  
  end func;

## [Sidef](http://rosettacode.org/wiki/Category:Sidef "Category:Sidef")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=76 "Edit section: Sidef")]

'*.p[lm]'.glob.each { |file| say file }    # Perl files under this directory

Output:

x.pl
x.pm

func file_match(Block callback, pattern=/\.txt\z/, path=Dir.cwd) {  
    path.open(\var dir_h) || return nil  
    dir_h.entries.each { |entry|  
        if (entry.basename ~~ pattern) {  
            callback(entry)  
        }  
    }  
}  
   
file_match(  
    path: %d'/tmp',  
    pattern: /\.p[lm]\z/i,  
    callback: { |file|  
        say file;  
    }  
)

Output:

/tmp/x.pl
/tmp/x.pm

## [Smalltalk](http://rosettacode.org/wiki/Category:Smalltalk "Category:Smalltalk")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=77 "Edit section: Smalltalk")]

(Directory name: 'a_directory')  
  allFilesMatching: '*.st' do: [ :f | (f name) displayNl ]

## [Tcl](http://rosettacode.org/wiki/Category:Tcl "Category:Tcl")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=78 "Edit section: Tcl")]

For the current directory:

foreach filename [glob *.txt] {  
    puts $filename  
}

For an arbitrary directory:

set dir /foo/bar  
foreach filename [glob -directory $dir *.txt] {  
    puts $filename  
    ### Or, if you only want the local filename part...  
    # puts [file tail $filename]  
}

## [Toka](http://rosettacode.org/wiki/Category:Toka "Category:Toka")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=79 "Edit section: Toka")]

As with the C example, this uses a a POSIX extended regular expression as the pattern. The  **dir.listByPattern**  function used here was introduced in library revision 1.3.

needs shell  
" ."  " .\\.txt$" dir.listByPattern

## [TUSCRIPT](http://rosettacode.org/wiki/Category:TUSCRIPT "Category:TUSCRIPT")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=80 "Edit section: TUSCRIPT")]

$$ MODE TUSCRIPT  
files=FILE_NAMES (+,-std-)  
fileswtxt= FILTER_INDEX (files,":*.txt:",-)  
txtfiles= SELECT (files,#fileswtxt)

Output:

files     DEST'MAKROS'ROSETTA.TXT'SKRIPTE'STUDENTS.XML'TUSTEP.INI
fileswtxt 3
txtfiles  ROSETTA.TXT

## [TXR](http://rosettacode.org/wiki/Category:TXR "Category:TXR")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=81 "Edit section: TXR")]

#### Using  `glob`[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=82 "Edit section: Using glob")]

(glob "/etc/*.conf")

Output:

("/etc/adduser.conf" "/etc/apg.conf" "/etc/blkid.conf" "/etc/brltty.conf"  
 "/etc/ca-certificates.conf" "/etc/colord.conf" "/etc/ddclient.conf"  
 "/etc/debconf.conf" "/etc/deluser.conf" "/etc/dnsmasq.conf" "/etc/ffserver.conf"  
 "/etc/fuse.conf" "/etc/gai.conf" "/etc/hdparm.conf" "/etc/host.conf"  
 "/etc/insserv.conf" "/etc/irssi.conf" "/etc/kernel-img.conf"  
 "/etc/kerneloops.conf" "/etc/knockd.conf" "/etc/ld.so.conf" "/etc/lftp.conf"  
 "/etc/logrotate.conf" "/etc/ltrace.conf" "/etc/mke2fs.conf" "/etc/mtools.conf"  
 "/etc/netscsid.conf" "/etc/nsswitch.conf" "/etc/ntp.conf" "/etc/pam.conf"  
 "/etc/pnm2ppa.conf" "/etc/popularity-contest.conf" "/etc/resolv.conf"  
 "/etc/rsyslog.conf" "/etc/sensors3.conf" "/etc/sysctl.conf" "/etc/ucf.conf"  
 "/etc/updatedb.conf" "/etc/usb_modeswitch.conf" "/etc/wodim.conf")

#### Using  `open-directory`  and  `get-lines`[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=83 "Edit section: Using open-directory and get-lines")]

(mappend [iff (op ends-with ".conf") list] (get-lines (open-directory "/etc")))

Output:

("ddclient.conf" "gai.conf" "ucf.conf" "kernel-img.conf" "ltrace.conf"  
 "debconf.conf" "apg.conf" "adduser.conf" "mke2fs.conf" "colord.conf"  
 "kerneloops.conf" "fuse.conf" "hdparm.conf" "irssi.conf" "host.conf"  
 "ffserver.conf" "pam.conf" "sysctl.conf" "ld.so.conf" "dnsmasq.conf"  
 "insserv.conf" "brltty.conf" "deluser.conf" "netscsid.conf" "nsswitch.conf"  
 "mtools.conf" "wodim.conf" "updatedb.conf" "popularity-contest.conf"  
 "knockd.conf" "ntp.conf" "sensors3.conf" "resolv.conf" "blkid.conf"  
 "lftp.conf" "ca-certificates.conf" "usb_modeswitch.conf" "logrotate.conf"  
 "rsyslog.conf" "pnm2ppa.conf")

## [UNIX Shell](http://rosettacode.org/wiki/Category:UNIX_Shell "Category:UNIX Shell")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=84 "Edit section: UNIX Shell")]

ls -d *.c                # *.c files in current directory  
(cd mydir && ls -d *.c)  # *.c files in mydir

`*.c`  is a  _file name pattern_, also known as a  _glob pattern_. The shell expands each pattern to a sorted list of matching files. Details are in your shell's manual.

If there are no *.c files,  `ls`  fails with an error message.

## [UnixPipes](http://rosettacode.org/wiki/Category:UnixPipes "Category:UnixPipes")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=85 "Edit section: UnixPipes")]

Here using grep for regexp.

ls | grep '\.c$'

## [VBScript](http://rosettacode.org/wiki/Category:VBScript "Category:VBScript")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=86 "Edit section: VBScript")]

   
Sub show_files(folder_path,pattern)  
	Set objfso = CreateObject("Scripting.FileSystemObject")  
	For Each file In objfso.GetFolder(folder_path).Files  
		If InStr(file.Name,pattern) Then  
			WScript.StdOut.WriteLine file.Name  
		End If  
	Next  
End Sub  
   
Call show_files("C:\Windows",".exe")  
 

## [Visual Basic .NET](http://rosettacode.org/wiki/Category:Visual_Basic_.NET "Category:Visual Basic .NET")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=87 "Edit section: Visual Basic .NET")]

**Works with**:  [Visual Basic .NET](http://rosettacode.org/wiki/Visual_Basic_.NET "Visual Basic .NET")  version 9.0+

'Using the OS pattern matching  
For Each file In IO.Directory.GetFiles("\temp", "*.txt")  
  Console.WriteLine(file)  
Next  
   
'Using VB's pattern matching and LINQ  
For Each file In (From name In IO.Directory.GetFiles("\temp") Where name Like "*.txt")  
  Console.WriteLine(file)  
Next  
   
'Using VB's pattern matching and dot-notation  
For Each file In IO.Directory.GetFiles("\temp").Where(Function(f) f Like "*.txt")  
  Console.WriteLine(file)  
Next

## [zkl](http://rosettacode.org/wiki/Category:Zkl "Category:Zkl")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=88 "Edit section: zkl")]

Unix glob, with wildcarding and options on file type, case folding and a few others.

File.glob("*.zkl") //--> list of matches

## [Zsh](http://rosettacode.org/wiki/Category:Zsh "Category:Zsh")[[edit](http://rosettacode.org/mw/index.php?title=Walk_a_directory/Non-recursively&action=edit&section=89 "Edit section: Zsh")]

Zsh has powerful filename generation features, which can filter by file names, permissions, size, type, etc.

print -l -- *.c

[Categories](http://rosettacode.org/wiki/Special:Categories "Special:Categories"):

-   [Programming Tasks](http://rosettacode.org/wiki/Category:Programming_Tasks "Category:Programming Tasks")
-   [File System Operations](http://rosettacode.org/wiki/Category:File_System_Operations "Category:File System Operations")
-   [68000 Assembly](http://rosettacode.org/wiki/Category:68000_Assembly "Category:68000 Assembly")
-   [8th](http://rosettacode.org/wiki/Category:8th "Category:8th")
-   [Ada](http://rosettacode.org/wiki/Category:Ada "Category:Ada")
-   [ALGOL 68](http://rosettacode.org/wiki/Category:ALGOL_68 "Category:ALGOL 68")
-   [AppleScript](http://rosettacode.org/wiki/Category:AppleScript "Category:AppleScript")