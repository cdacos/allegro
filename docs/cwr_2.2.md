Page 1 of 80

#### CWR1 9 - 1070

**Information Services Committee
Conference Call, 05 / 11 /201 9**
Source Language: English
Published on: 05 / 11 /201 9

# Functional specifications:

# Common Works Registration

# Version 2.2 – Revision 2

Society Publisher Forum

Status: ADOPTED by the Information Services Committee (Conference call, 05 / 11 /201 9 )
(ISC17- 1284 )

Date: 05 / 11 / 2019

Access rights: External to CISAC


**Latest Revisions and updates**
(Previous revisions available at the end of the document)

```
Number of revision Date Main modifications
2.2 05/
```
#### 14/04/

```
➢ HDR: new fields for, version, revision, software package and
package release number
➢ SPU: IPI Name Numbers made mandatory for a submitter’s own
IPI Name Numbers
➢ PWR removed edit to allow PWR to follow OWR
➢ PWR to SPU Sequence Number link
➢ OPT New ‘non-controlled publisher collection’ record (optional)
➢ OWT new ‘other writer collection’ record (optional)
➢ REC New optional fields for Sound Recording Title, Sound
Recording version title, Record Label, Display Artist
➢ ORN removed previously invalid ISAN field and note relating to
its use
➢ ORN New optional Field for ISAN and EIDR
➢ XRF new Optional Record
➢ Society specific edits agreed at the New York meeting added
➢ Information Services Committee (Lisbon, 14/04/2016) approves
Version 2.
2.2R 1 25.04.2017 ➢ Correction of missing edit PWRF006 and renumbering the edits
below.
2.2R 2 16 /09/2019 ➢ Update to transmission header record to account for IPI Name
Numbers (IPNN) greater than 9 digits.
➢ OPU validation updates.
➢ GRH record version number update.
➢ MSG record example removal.
➢ Addition of missing XREF record in Transaction Format table and
file skeletons.
➢ Update to XRF field description for Organisation Code.
➢ Approved by the ISC conference call on 05/11/
```
Summary: This document contains the technical description of the Common Works Registration version 2.
Rev 2. This version of CWR contains minor changes (as outlined above) from the Common Works Registration
version 2.1 Rev 7 that has been approved by the Information Services Committee (Lisbon, 14/04/2016). The
format of transactions and records are included.

Send comments/questions regarding this manual to **CWR@cisac.org** or any member of the CWR
Management Committee.


## TABLE OF CONTENTS


- 1 Introduction
- 2 EDI Terminology
    - 2.1 Properties of EDI Components
- 3 File Structure
    - 3.1 File Naming Convention
    - 3.2 Control Records
    - 3.3 Notification of Data Transmission & Acknowledgement of Notification/Validation Status
    - 3.4 File Level Validation
    - 3.5 HDR: Transmission Header.....................................................................................................
    - 3.6 GRH: Group Header
    - 3.7 GRT: Group Trailer
    - 3.8 TRL: Transmission Trailer
- 4 Transaction Header Records
    - 4.1 AGR: Agreement Supporting Work Registration
    - 4.2 NWR: New Work Registration
    - 4.3 REV: Revised Registration
    - 4.4 ISW: Notification of ISWC assign to a work
    - 4.5 EXC: Existing work which is in Conflict with a Work Registration
    - 4.1 ACK: Acknowledgement of Transaction
- 5 Detail Records
    - 5.1 TER: Territory in Agreement
    - 5.2 IPA: Interested Party of Agreement
    - 5.3 NPA: Non-Roman Alphabet Agreement Party Name
    - 5.4 SPU: Publisher Controlled By Submitter
    - 5.5 OPU: Other Publisher
    - 5.6 NPN: Non-Roman Alphabet Publisher Name
    - 5.7 SPT: Publisher Territory of Control
    - 5.8 OPT: Publisher Non-Controlled Collection
    - 5.9 SWR: Writer Controlled By Submitter
    - 5.10 OWR: Other Writer
    - 5.11 NWN: Non-Roman Alphabet Writer Name
    - 5.12 SWT: Writer Territory of Control
    - 5.13 OWT: Other Writer Collection
    - 5.14 PWR: Publisher For Writer
    - 5. 15 ALT: Alternate Title................................................................................................................
    - 5.16 NAT: Non-Roman Alphabet Title
    - 5.17 EWT: Entire Work Title for Excerpts
    - 5.18 VER: Original Work Title for Versions
    - 5.19 PER: Performing Artist
    - 5.20 NPR: Performance Data in non-roman alphabet
    - 5.21 REC: Recording Detail
    - 5.22 ORN: Work Origin
    - 5.23 INS: Instrumentation Summary
    - 5.24 IND: Instrumentation Detail
    - 5.25 COM: Component
    - 5.26 MSG: Message
    - 5.27 NET: Non-Roman Alphabet Entire Work Title for Excerpts
    - 5.28 NCT: Non-Roman Alphabet Title for Components
    - 5.29 NVT: Non-Roman Alphabet Original Title for Version
    - 5.30 NOW: Non-Roman Alphabet Other Writer Name
    - 5.31 ARI: Additional Related Information
    - 5.32 XRF: Work ID Cross Reference
- 6 CWR Data Structure
    - 6.1 Definition of CWR using BNF
    - 6.2 File skeleton sent from publisher to societies
    - 6.3 File skeleton sent from society to publisher
    - 6.4 File skeleton containing Non-Roman Alphabet Records
- 7 Society-Specific Edits
    - 7.1 ABRAMUS and UBC
    - 7.2 ASCAP
    - 7.3 BMI
    - 7.4 GEMA
    - 7.5 Harry Fox
    - 7.6 ICE Societies
    - 7.7 MusicMark
    - 7.8 SACEM
    - 7.9 SESAC
    - 7.10 SGAE
    - 7.11 SIAE
- 8 Previous Revisions


## 1 Introduction

CWR v2 was adopted as a CISAC standard by the CIS Steering Committee in its meeting February 27, 28 2001
in Madrid.

The purpose of the Common Works Registration (CWR) format is to provide publishers and societies with a
standard format for the registration of works. This provides many benefits for both publishers and societies.
Publishers will be able to create one registration file and send it to participating societies around the world.
Each society that receives that file will follow a similar process, and provide acknowledgement in a common
format back to the publisher. Similarly, as more and more publishers adopt the CWR format, societies will
receive the majority of the work registrations in the one standard format. This will result in significant savings
in time and resources.

Another benefit of the CWR is that it is flexible enough to provide for all of the data elements that rights
organizations require in a registration for a work. Some of these data elements may not be available at
present, but they can be added to the database of publishers and societies alike so that in the future this
data can be captured. For some data elements, the CWR provides a means of capturing those data elements.
For example, the societies will provide IPI numbers for participants and ISWCs for works in their
acknowledgement records whenever possible. The publishers can then populate their databases with this
additional data.

The electronic submission of unique identifiers such as the ISWC and IPI will provide a much more efficient
way for the societies and publishers to accurately and quickly identify works and interested parties.

The CWR employs data standards that have been developed for the Common Information System (CIS)
project of CISAC. Using existing standards for codes will eliminate confusion. Version 1.1 of Standards for
Electronic Data Interchange and Communications developed in CIS is used as much as possible (based on the
document IS/IM/47). This provides a solid foundation for the CWR.

Version 2 of the CWR format includes an option to include agreement transactions. Agreement transactions
can be used to refer to an existing agreement or to inform a society of a specific agreement. This transaction
is not meant as a vehicle for the notification of agreements - it is meant to provide agreement information
for the works that are being registered.

Version 2 of the CWR format includes records to allow titles and names in Chinese, Japanese, Korean, and
other non-Roman alphabets. Separate records have been provided so that societies and publishers that
cannot deal with non-Roman alphabets may simply ignore the data.

Version 2.1 of the CWR format includes additional fields for the registration of cues, and a new record, ARI.
For a less technical description of the CWR format as well as examples, please see the CWR User Manual.

Related Documents are:

- CRF0028 – CWRv2.1 User Manual
- CRF020 – CWR Validation and Lookup Tables
- IM0047 – EDI Standards
- **CWR06- 1972** – CWR Sender ID and Codes
- **CWR08- 2493** – CWR Error Messages

The latest version of all the documents can be found on the CISAC website: **[http://www.cisac.org](http://www.cisac.org)**.


### Standards applicable to all CIS transactions

_This section has been extracted from the CIS Guidelines for Electronic Data Interchange (IS/IM/47). The
guidelines were developed by the International Modelling Group, and approved by the CISAC community for
general use by all societies for all types of data transmission._

## 2 EDI Terminology

**Data Element:** the basic unit of information in the EDI standard. Data elements (also referred to as Fields)
contain information that represents a singular fact, for example, song title, date of birth, or production year.
Data elements can be codes, numeric values, or literal descriptions. Data element definitions must include a
description, specifications for data type, size, and if appropriate, a list of code values and definitions of each
value.

**Detail Record:** a combination of functionally related data elements. An identifier or record type is placed at
the beginning of each detail record identifying its purpose. Detail record definitions must include a
description, the list of data elements that are included in the detail record along with an indication of whether
each data element is mandatory, optional, or conditional, and the validation rules to insure all data elements
are correct.

**Transaction Header:** a combination of functionally related data elements that is also used to define the start
of a business transaction. An identifier or transaction code is placed at the beginning of each transaction
header identifying its purpose. Transaction header definitions must include a description, the list of data
elements that are included in the transaction header record along with an indication of whether each data
element is mandatory, optional, or conditional, and the validation rules to insure all data elements are
correct.

**Transaction:** a transaction header that may or may not be followed by any number of supporting detail
records. A transaction contains all the data required to define a specific business transaction e.g. transactions
may represent the equivalent of

- Work Registrations
- Agreements
- Interested Party Information

Transaction definitions must include a list of what detail records can be included in the transaction along with
an indication of whether the detail record is mandatory, optional, or conditional. For each of those detail
records, the definition must also indicate the repeat frequency for the record (how many times can this
record occur within this transaction).

**Group:** composed of one or more transactions of the same type. Each group begins with a header record that
identifies the type of transaction contained in the group, and ends with a trailer that includes control totals
summarizing the content of the file. Note that a group can contain up to 10,000,000 transactions.

**File:** composed of one or more groups. Files are the unit that ends up getting transmitted between CISAC EDI
participants. Each file begins with a header record that identifies the file’s origination and ends with a trailer
that includes control totals summarizing the content of the file.

**Control Records:** provide information about the content of a group or file. These records insure that only
authorized users of this facility are participating and that the integrity of each transaction is maintained as
the file travels through various telecommunication lines.


### 2.1 Properties of EDI Components

Files transmitted within this standard must exhibit the following characteristics:

- All data will be ASCII encoded. The valid ASCII characters are as defined in CIS character set SR06- 1311 ,
  which is available on the CISAC Website [http://www.cisac.org.](http://www.cisac.org.)
  _Note:_ The CWR MC has allowed for other character sets to be used (to permit the registration of certain
  works) if a character set is noted in the HDR record.
- Records are variable length and carriage return / line feed (CR/LF) delimited
- Fields within the records are fixed length and are not delimited
- All alphabetic characters will be presented in upper case

The standard for transmission will be the File Transfer Protocol (FTP). Each participant will have to provide
the address for its public FTP site and each transmission will be a push (i.e. the sender will deliver its file to
the recipients public FTP site).

**Data Element Properties**

The following information will be defined for each data element within the CISAC EDI standard:

- **Field:** Indicates the name of the data element
- **Start:** The position in the record where this field starts (note the first position of the record is “1”).
- **Size:** The number of characters that this field will occupy.
- **Format:** The type of data included in this field. Data present in a field that is inconsistent with the rules
  defined below will cause that record (and potentially the entire transaction) to be rejected. The legend
  of type abbreviations is...

```
Code Description Default Size Rules
```
```
A Alpha or Alphanumeric None
```
```
Any string containing valid ASCII text. Note that nulls are not
acceptable and all alphabetic characters must be in upper case. If
there is no data to be entered in an alpha field, blanks must be
entered.
B Boolean 1 Field must be equal to <Y>es or <N>o
```
```
F Flag 1
```
```
Field must be equal to <Y>es, <N>o, or <U>nknown. Note that the
difference between Boolean and Flag is the allowance of an
unknown condition for all fields declared as Flag.
```
```
D Date 8 Dates are all formatted as YYYYMMDD. If there is no data to be entered in a date field, zeroes must be entered.
```
```
N Numeric None
```
```
Numeric fields are to be right justified and zero filled. If there is an
implied decimal point, it will be defined in the record layout. If there
is no data to be entered in a numeric field, zeroes must be entered.
```
```
T Time or Duration 6
```
```
Time/Duration fields are all formatted as HHMMSS. Time of day is to
be expressed in 24 hour format, otherwise known as military time. If
there is no data to be entered in a time or duration field, zeroes must
be entered.
```
```
L List or Table Lookup None
```
```
The valid entries for these fields come from a list in the field
description or a table. Note that the values for these tables are to be
found in the Lookup Table document.
```

- **Req:** Indicates whether or not an entry must be present in this field. Values in the REQ field will be...

```
 M: Mandatory, meaning this field must be filled out. If it is not filled out, this record will be
rejected and, depending on the record type, the entire transaction may be rejected. Note that
not all record types are mandatory; however, there are mandatory fields within optional records.
```
```
 C: Conditional, meaning this field may be Mandatory depending on other conditions that exist in
either the current record or the transaction. If the condition results in a mandatory field and this
field is not present, this record will be rejected and, depending on the record type, the entire
transaction may be rejected.
```
```
 O: Optional, meaning this field may or may not be entered.
```
- **Field Description:** Provides a basic description of the field to be entered. Also included will be the
  individual table where valid entries reside for fields with a format type equal to “L”.

**Detail Record Properties**

The following information will be defined for each detail record within the CISAC EDI standard:

- **Record Description:** Provides a detailed description of the purpose of this record.
- **Record Format:** Lists the data elements from which this detail record is composed. Each data element
  definition within the Record Format section will include all Data Element Properties as listed above.
- **Record Level Validation:** The validation criteria that will be applied by the recipient to this detail record
  when received. Record level validation insures validity of this detail record.
- **Field Level Validation:** The validation criteria that will be applied by the recipient to this detail record
  when received. Field level validation insures the validity of each data element contained in the detail
  record.

**Transaction Header Properties**

The following information will be defined for each transaction header within the CISAC EDI standard:

- **Transaction Description:** Provides a detailed description of the purpose of this transaction.
- **Transaction Format:** Lists the transaction header and the various detail records (if any) from which this
  transaction is composed. For each detail record, three additional items are defined...

```
➢ Req: indicates whether the detail record/transaction header is Required (R), Optional (O), or
Conditional (C).
```
```
➢ Max Use: Indicates the number of times this detail record can appear within a transaction. Values
are either 1 meaning the record can only occur once, or M meaning the record can appear as many
times as is required.
```
```
➢ Comment: Used to communicate any additional information which may be helpful to those
implementing the transaction.
```
- **Record Description:** Provides a detailed description of the purpose of this transaction header.
- **Record Format:** Lists the data elements from which this detail record is composed. Each data element
  definition within the Record Format section will include all Data Element Properties as listed above.
- **Transaction Level Validation:** The validation criteria that will be applied by the recipient to this detail
  record when received. Transaction level validation insures validity of this detail record as it relates to the
  overall transaction.


- **Field Level Validation:** The validation criteria that will be applied by the recipient to this detail record
  when received. Field level validation insures the validity of each data element contained in the detail
  record.

**Validation**

Subsequent to each detail record or transaction header description, a set of validation criteria will be
provided. These criteria are listed at different potential levels depending on the record being edited. The
levels of validation are File, Group, Transaction, Record, or Field. As a result of validation, the same levels of
detail may be rejected from the data. Rejections are indicated at the end of the validation criteria with one
of the following codes...

- **ER:** Entire File is rejected
- **GR:** Entire Group is rejected
- **TR:** Entire Transaction is rejected
- **RR:** Entire Record is rejected
- **FR:** Field is rejected and a default value is specified for the field

**Record Prefixes**

Each Transaction Header and Detail Record contains a prefix that identifies both the record and the
transaction that is being delivered. The attached table describes the layout of the prefix area...

```
Field Start Size Fmt Req Field Description
Record Type 1 3 L M The three character transaction type or detail record type.
These values reside in the Record Type Table.
Transaction
Sequence #
```
```
4 8 N M If this is the first transaction within a group, the Transaction
Sequence # must be equal to 00000000. Otherwise, for
transaction headers, the Transaction Sequence # must be equal
to the previous transaction header’s Transaction Sequence #
incremented by 1. For detail records, the Transaction Sequence
# must be equal to the Transaction Sequence # of the previous
transaction header.
Record Sequence
#
```
```
12 8 N M For transaction headers, always set to 00000000. For detail
records, set this field to the Record Sequence # of the previous
record written to the file incremented by 1.
```
**Field Level Validation**

1. _Record Type_ must be either a valid transaction type or a valid detail record type. (ER)
2. If this is the first transaction header in the group, _Transaction Sequence #_ must be equal to 0. (ER)
3. If this is a transaction header that is not the first transaction header in the group, the Transaction
   Sequence # must be equal to the previous transaction’s Transaction Sequence # incremented by 1. (TR)
   [1]
4. If this is a detail record, the Transaction Sequence # must be equal to the previous record’s Transaction
   Sequence #. (TR) [1]
5. If this is a transaction header record, the _Record Sequence #_ must be equal to zero. (ER)
6. If this is a detail record, the _Record Sequence_ # must be equal to the previous record’s _Record Sequence_
   _#_ incremented by 1. (ER)


7. If the _Transaction Sequence #_ on subsequent transactions are not in sequential order within a group, the
   entire file will be rejected. (ER)
8. If any detail records belonging to a transaction header do not carry the same _Transaction Sequence #_ as
   the preceding transaction header, the subordinate records are out of sequence. In this case, the entire
   file will be rejected. (ER)
9. Record length must match the record length specified within the specification. (ER)

**[1] Note:** When the NWR transaction follows an ACK transaction it is regarded as a continuation of the same
transaction (since the acknowledgement is for the work described in the NWR). Therefore the NWR has the
same transaction number as the ACK, and the record sequence numbers continue to be incremented by one.
Similarly when the EXC transaction follows an NWR, it will contain the same transaction number as the NWR
and the record sequence numbers continue to be incremented by one.

## 3 File Structure

Note that the File Naming Convention has been modified by the CWR Management Committee to better suit
the needs of CWR. It no longer conforms to the convention specified in the EDI Standards.

### 3.1 File Naming Convention

As of August 2006 the file convention of CWyynnnnsss_rrr.Vxx was adopted for use in files sent by publishers
to societies and vice versa where

```
CW – identifies a CWR file
```
```
yy – identifies the year
```
```
nnnn – is the sequence # assigned by the publisher
```
```
sss – is the sender (2 or 3 char code for publisher, or the 3 digit code for society)
```
```
rrr – is the receiver (2 or 3 char code for publisher, or the 3 digit code for society)
```
```
Vxx – is the version
```
This replaces the previous file naming convention, which was exactly the same except that the file sequence
number only had 2 digits instead of 4.

If the file is zipped, it will be named CWyynnnnsss_rrr.zip. The unzipped file it contains will be named as
above with the version number.

Note that if the same file is being sent to several societies, use ‘000’ as the society code.

EXAMPLE:

CW060001EMI_044.zip would be the name of the first file sent by EMI to MCPS-PRS in 2006. This file is
zipped. When it is unzipped, the file name would be CW060001EMI_044.V21 indicating the data was in
CWRv2.1.


### 3.2 Control Records

The following record layouts are used to partition and control the submission of files between participants.
Proper control records are required within the file to insure the integrity of transmission over
telecommunication lines, as well as confirming that the data within the file has not been altered as a result
of intentional or unintentional tampering with data.

Control records defined within this version of the standard are...

 **HDR:** Transmission Header

 **GRH:** Group Header

 **GRT:** Group Trailer

 **TRL:** Transmission Trailer

### 3.3 Notification of Data Transmission & Acknowledgement of Notification/Validation Status

**Data Transmission Form**

When a publisher or society sends a transaction file to a recipient for processing, the sender also notifies the
recipient of the transmission in an email. The suggested format of the data transmission notification email is
shown below. The recipient then removes the file from the ftp location, validates and processes it.

```
Notification Of Data Transmission
```
_This form should be used to notify the intended recipient that a file has either been sent to them, or is ready
to be retrieved._

**From**

```
Sending Entity
Sender
```
**File Details**

Filename
Location
Description
File size
Date/Time Stamp
Number of Transactions
Number of Records
**Notification Details**

```
Date/Time of
Notification
```
```
Re-Notification YES/NO
```

**Explanation:**

```
Sending Entity Name of Society or Publisher sending the Data
Sender Name of person responsible for data exchange. Receipt of
data or any problems will need to be sent to this person
Filename Filename of data file being sent. Should be from list of agreed
names
Location FTP location from which file can be retrieved. This should be of the
form:
ftp: // prs.co.uk/ftp/incoming where prs.co.uk is the FTP site name
ftp/incoming is the full hierarchical directory name of where the
file is placed. If file sent as e-mail attachment, just enter "e-mail "
Description Content of file
File size In bytes as indicated by the "DIR " command
Date/Time
Stamp
```
```
As indicated by "DIR" command.
```
```
Number of
Transactions
```
```
Number of transactions e.g. number of works added or modified in
each group
Number of
Records
```
```
Number of records added or modified in the file
```
```
Date/Time of
Notification
```
```
Date and time when notification sent
```
```
Re-Notification YES/NO to indicate if a previous notification has been sent
```
**Note:** The standard for dates (YYYYMMDD) and time (HHMMSS) should be used when completing this form.

**Data Acknowledgement Form**

After the transaction file has been processed, the recipient notifies the sender by email that the
acknowledgement file is available. The format of the data acknowledgement email is shown below. The
acknowledgment file can then be removed from the ftp location and processed. After processing the file
should be deleted from the ftp directory.

```
Acknowledgement Of Notification/Validation Status
```
**From**

```
Society
Sender
```
**File Details**

```
Filename
Location
Description
File size
Date/Time Stamp
Number of Transactions
Number of Records
```

**Status**

```
The above file has been received and is awaiting validation/processing YES/NO
The above file has been received and has been successfully
validated/processed
```
#### YES/NO

```
The above file is no longer required and can be deleted YES/NO
The above file has been received and has failed validation/processing.
Please send again. Details of failure as below
```
#### YES/NO

**Details of Failure**

**Note:** The standard for dates (YYYYMMDD) and time (HHMMSS) should be used when completing this
form.

### 3.4 File Level Validation

Along with the control records, a number of validation checks are performed at a file level. The editing criteria
for full file submission are listed below:

1. If the file cannot be read, the entire file will be rejected. (ER)
2. If the first record on the file is not record type HDR, the entire file will be rejected. (ER)
3. If the second record on the file is not record type GRH, the entire file will be rejected. (ER)
4. If every subsequent GRH on the file is not preceded by a GRT, the entire file will be rejected. (ER)
5. If the last record on the file is not record type TRL, the entire file will be rejected. (ER)
6. If record type GRH is not followed by a transaction header record type, the entire file will be rejected.
   (ER)
7. If record type GRT is not followed by a record type GRH or TRL, the entire file will be rejected. (ER)
8. If the file contains more than one record type HDR or TRL, the entire file will be rejected. (ER)
9. If the header (HDR) contains an invalid version number the entire file will be rejected (ER)
10. If the header (HDR) contains a invalid revision number the entire file will be rejected (ER)


### 3.5 HDR: Transmission Header.....................................................................................................

**Record Description**

This is a required “cover sheet” for transmissions submitted by a participant. It will contain the file control
information as well as the name of the submitter.

The character set field added for Version 2.1 is simply intended to be a way of informing societies that there
is a non-ASCII character set (such as Chinese Characters) used somewhere in the file. Such files are only
intended to be sent to societies that accept and use such character sets (e.g. CASH), and the value in the field
will inform those societies which character set has been used. The list of the relevant character sets is
currently being developed and will appear in the lookup tables once it is ready. If such a file is sent to a society
that does not accept non-ASCII characters then it should get rejected in the normal way during the file
validation process.

If a publisher must send a CWR Sender ID (IPNN) greater than 9 digits, then as a workaround the submitting
publisher can use the existing Sender Type field to provide the leading two numbers of the CWR Sender ID
(IPNN) and use the existing Sender ID field to provide the remaining 9 digits (2 + 9 = 11 Digits). This potential
workaround should be discussed between the submitting publisher and the receiving societies prior
implementation since the receiving societies will have to accept a numeric value in place of the Sender Type
and concatenate the Sender Type and Sender ID fields to render the 11 CWR Sender ID (IPNN).

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Type 1 3 A M HDR = Transmission Header
Sender Type 4 2 A M Indicates if the sender of the file is a society or a publisher.
Values are PB = Publisher, SO = Society, AA = Administrative
Agency, WR = Writer. If CWR Sender ID (IPNN) greater than 9
digits (without the 2 leading 0) then use Sender Type field to
provide leading numbers of the CWR Sender ID.
Sender ID 6 9 N M If Sender Type is equal to PB, AA, or WR, the sender must enter
their assigned CWR IPI # in this field. These values reside in the
CWR Sender ID and Codes Table. If Sender Type is equal to SO,
the sending society must enter their Society Code. These values
reside in the Society Code Table. If CWR Sender ID (IPNN) greater
than 9 digits then use Sender ID to provide remaining numbers
of the CWR Sender ID.
Sender Name 15 45 A M The name of the sender (publisher, society, agency)
EDI Standard
Version Number
```
```
60 5 A M Indicates which version of the header and trailer records was
used to create the data in this file. This field must be set to 01.
for this version of the standard.
Creation Date 65 8 D M The date that this file was created.
Creation Time 73 6 T M The time of day that this file was created.
Transmission
Date
```
```
79 8 D M The date that this file was transmitted to all receiving entities.
```
**_Field Added for CWRv2._**

```
Character Set 87 15 L O To be used if this file contains data in a character set other than
ASCII.
```

**_Field Added for CWRv2._**

```
Version 102 3 A O CWR version (2.2)
```
```
Revision 105 3 N O CWR revision number
```
```
Software Package 108 30 A O Name of the software package from which the file originated
```
```
Software Package
Version
```
```
138 30 A O Version of the software package from which the file originated
```
**Field Level Validation**

1. _Record Type_ must be equal to HDR. (ER)
   _2. Sender Type_ must be equal to PB (publisher), SO (society), WR (writer), or AA (administrative agency)
   except where sender needs to use the Sender Type field to supply the leading 2 numbers of their
   IPNN. (ER)
3. If _Sender Type_ is equal to PB, WR, or AA, _Sender ID_ must be entered and must match the assigned
   entry in the CWR Sender ID and Codes Table and if _Sender Type_ has the leading 2 numbers of the
   Sender’s IPNN, then _Sender Type_ plus _Sender ID_ must match the assigned entry in the CWR Sender
   ID and Codes Table. (ER)
4. If _Sender Type_ is equal to SO, _Sender ID_ must be entered and must match an entry in the Society Code
   Table. (ER)
5. If _Sender Type_ is equal to PB, _Sender Name_ must match the name on the corresponding entry in the
   CWR Sender ID and Codes Table. (ER)
6. If _Sender Type_ is equal to SO, _Sender Name_ must match the name on the corresponding entry in the
   Society Code Table. (ER)
7. If _Sender Type_ is equal to AA, _Sender ID_ must contain the IPI# of the Publisher that the Administrative
   Agency is acting on behalf of. Note that transactions for multiple submitting publishers cannot be co-
   mingled in a single file. (ER)
8. If _Sender Type_ is equal to AA, _Sender Name_ must match the name on the corresponding entry in the
   Publisher Code Table. (ER)
9. _EDI Standard Version Number_ must be equal to the constant value “01.10”. (ER)
10. _Creation Date_ must be a valid date. (ER)
11. _Transmission Date_ must be a valid date. (ER)
12. If the _Sender Type_ is PB, the _Sender ID_ must be for an approved CWR participant. (ER)
13. If the _Sender Type_ is equal to WR _, Sender ID_ must be a valid IPI # for a writer. (ER)

_Edit for CWRv2._

14. If entered, the _Character Se_ t must be one of Traditional [Big5] or Simplified [GB] or a value from the
    Unicode table, UTF-8 (reference [http://www.unicode.org/charts)](http://www.unicode.org/charts)) (ER)

_Edit for CWRv2._


15. Version if entered and must be 2.2 (ER)
16. Revision number if entered must be a valid CWR version 2.2 revision number from the version
    number lookup table, the current value must be 1 (for this revision 1) (ER)

### 3.6 GRH: Group Header

**Record Description**

The GRH record is used to indicate the presence of a group (or batch) of transactions within the file. A group
can only contain one type of transaction and this is indicated in the Transaction Type field. Also all
transactions of the same type should be contained in the same group (e.g. all NWR transactions should
appear in one single NWR group) and each group type can only be used once per file (i.e. there can only be
one NWR and one REV group per file)

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Type 1 3 A M GRH = Group Header
Transaction Type 4 3 L M Indicates the type of transactions included in this group. Values
for this field reside in the Transaction Type table.
Group ID 7 5 N M A unique sequential number for this group within this file. Group
ID should start at 00001.
```
**_Version 2 .10 fields_**

```
Version Number
for this
transaction type
```
```
12 5 A M Indicates the version of the previously referred to transaction
type that will follow within this group. For CWR version 2.1, set
to 02.
Batch request 17 10 N O A unique sequential number to identify the group. This number
is managed by the submitter to identify the group among
multiple submission files.
Submission/
Distribution type
```
```
27 2 L C Set to blank - Not used for CWR
```
**Field Level Validation**

1. _Transaction Type_ must be entered and must match an entry in the Transaction Type table. (GR)
2. _Group ID_ must be entered, must start at 1, and must increment by 1 sequentially for each new group in
   the file. (GR)
3. GRH records must follow either a GRT record or an HDR record. (ER)
4. For use of the CWR version 2 as described in this document, the Version Number must be '02. 2 0’. (GR)
5. Each Group Transaction type can only be used once per file. (GR)

Note: Submission / Distribution Type is used only in the case of audio-visual transactions. This field will be
ignored for CWR transactions.


### 3.7 GRT: Group Trailer

**Record Description**

The Group Trailer Record indicates the end of a group and provides both transaction and record counts for
the group.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Type 1 3 A M GRT = Group Trailer
Group ID 4 5 N M The same group id that was present on the preceding GRH
record.
Transaction
Count
```
```
9 8 N M The number of transactions included within this group.
```
```
Record Count 17 8 N M The number of physical records included within this group
including GRH and GRT records.
```
**_Version 1.10 fields – Not used for CWR_**

```
Currency
indicator
```
```
25 3 L C ISO Code of the currency used for the total monetary value. ISO
codes can be found in ISO 4217 table (actually ISO 3166 country
codes 2A plus the first letter of the currency of the country: for
example, USD for Dollar in US).
Total
monetary value
```
```
28 10 N O Total monetary value represented by the group.
```
**Field Level Validation**

1. _Group ID_ must be equal to the Group ID presented on the previous GRH record. (GR)
2. _Transaction count_ must be equal to the total number of transactions within this group. (GR)
3. _Record count_ must be equal to the total number of physical records inclusive of the GRH and GRT
   records. (GR)
4. _Currency Indicator_ is mandatory if _Total Monetary Value_ is provided (GR).

**Note:** Currency Indicator and Total Monetary Value will be ignored for CWR.

### 3.8 TRL: Transmission Trailer

**Record Description**

The Transmission Trailer record indicates the end of the transmission file. Control totals representing the
number of groups, transactions, and records within the file are included on this record.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Type 1 3 A M TRL = Transmission Trailer
Group Count 4 5 N M The number of groups included within this file.
Transaction
Count
```
```
9 8 N M The number of transactions included within this file.
```
```
Record Count 17 8 N M The number of physical records included in this file
including HDR and TRL records.
```

**Field Level Validation**

1. _Group Count_ must be equal to the number of groups within the entire file. (ER)
2. _Transaction count_ must be equal to the number of transactions within the entire file. (ER)
3. _Record count_ must be equal to the number of physical records inclusive of the HDR and TRL records. (ER)

## Standards for Work Registrations

## 4 Transaction Header Records

Within the Common Works Registration format, a transaction is defined as “all the information required to
complete a logical unit of work that is to be passed between two or more participants in the electronic
relationship.” This version of the standard contains definitions for the following transaction types:

 **AGR:** Agreement supporting Work Registration

 **NWR:** New Works Registration

 **REV:** Revised Registration

 **ISW:** Notification of ISWC assigned to a work

 **EXC:** Existing work which is in conflict with a work registration

 **ACK:** Acknowledgment of Transaction

Physically within the file, a transaction consists of a Transaction Header Record followed by the Detail Records
that further define the characteristics of the transaction. The content of each Transaction Header Record
along with valid combinations of detail records will be defined on the subsequent pages.

Please refer to section 6 for a definition of the structure of the CWR transactions using Backus Naur Form
(BNF). BNF is a notation for defining in an unambiguous way the relationships amongst the transactions.

### 4.1 AGR: Agreement Supporting Work Registration

The Agreement Supporting Work Registration (AGR) are transactions that may be used by publishers to
inform societies of the details of agreements relating to works that the publishers are registering elsewhere
in the same file using the NWR or REV transactions. The AGR can be used to document the agreement
between one or more writers and one or more publishers, or between a publisher and one or more
publishers. This transaction is not meant to be used to register general agreements.

The AGR contains a Submitter Agreement Number that is used to link the agreement to a work registration.
If a society has assigned an agreement number, then it too can be used as the link. The link is established as
follows:

- The writer to publisher agreement numbers are recorded in the record that links the writer to the
  publisher (PWR). The reason is that if two or more writers for a work have an agreement with the same
  original publisher, it is possible to record each Society-Assigned Agreement Number / Submitter
  Agreement Number in the PWR record that links that writer to the original publisher. If the original


```
Society-Assigned Agreement Number / Submitter Agreement Number were to be stored in the original
publisher record, then there is only place for one Society-Assigned Agreement Number / Submitter
Agreement Number.
```
- The publisher to publisher agreement numbers are recorded in the record for the sub-publisher or
  administrator.

It is the society of the original publisher that assigns the society-assigned agreement number to the writer to
publisher agreement. It is the society of the sub-publisher or the acquiring party that assigns the society-
assigned agreement number to the publisher to publisher agreement.

Where given, the territories and interested parties within the NWR or REV will be validated against those
entered in the corresponding AGR transaction.
**Transaction Format**

```
Record
Type
```
```
Name Req Max
Use
```
```
Comments
```
```
AGR Agreement Supporting Work
Registration
```
```
M 1 The first record within the AGR transaction must be an
AGR record.
TER Territory in Agreement M M Specifies the territories controlled by this agreement
```
```
IPA Interested Party of Agreement M M For each TER, lists all the Interested Parties (assignor and
acquirer(s)) associated with the territory.
NPA Non-Roman Alphabet
Agreement Party Name
```
```
O M If an assignor or acquirer has a non-Roman alphabet
name, enter it here.
```
**Record Description**

The AGR record contains basic information about an agreement that cover(s) work(s). Each AGR record must
be followed by a combination of at least one TER record and at least two IPA records. The shares within the
assignor and acquirer(s) records must total 100% for each type of right for each agreement. For further
examples, see the User Manual.

```
Record
Type
```
```
Agreement
Type
```
```
Territory Assignor/
Acquirer
```
```
IP’s Role Interested Party Performing
Right Share
```
Mechanical
Right Share
AGR OS
TER Europe
IPA Assignor CA Dupont 33,34 25
IPA Assignor CA Leblanc 33,33 25
IPA Acquirer E Warner France 33,33 50
**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = AGR (Agreement)
Submitter
Agreement
Number
```
```
20 14 A M The submitter’s unique identifier for this agreement.
```
```
International
Standard
Agreement Code
```
```
34 14 A O The ISAC that has been assigned to this agreement. Note this
code is not yet available.
```
```
Agreement Type 48 2 L M Code defining the category of agreement. The values reside in
the Agreement Type Table.
Agreement Start
Date
```
```
50 8 D M The date on which the transfer of rights to the acquiring party
becomes effective. This field is mandatory.
```

**Agreement End
Date**

58 8 D O The date on which the transfer of rights to the acquiring party
ceases except for the agreed provisions concerning entitlements
such as a collection.
**Retention End
Date**

```
66 8 D O The end date of the retention period. This date supersedes the
function of the Agreement End Date when a retention period is
part of the agreement. The Retention End Date must be equal to
or later than the Agreement End Date.
```
**Prior Royalty
Status**

```
74 1 A M Indicates whether or not the acquiring party is entitled to collect
monies that were accrued before the Agreement Start Date of
this agreement but not yet distributed by the societies. Possible
values are: "N"one, or "A"ll, or "D"ate (the latter indicating a
date is entered in Prior Royalty Start Date).
```
**Prior Royalty
Start Date**

```
75 8 D C Date before the Agreement Start Date of this agreement from
which royalties are accrued to which the acquiring party is
entitled to collect monies not yet distributed by societies. This
date must be entered if and only if the Prior Royalty Status is
'D'ate.
```
**Post-term
Collection Status**

```
83 1 A M Indicates whether the acquiring party is entitled to collect
monies that were accrued before the Retention End Date (if it
exists), or else the Agreement End Date but not yet distributed
by the societies. Possible values are "N"o, "O"pen-ended, or
"D"ate (the latter indicating a date is entered in Post-term
Collection End Date).
```
**Post-term
Collection End
Date**

```
84 8 D C The date until which the acquiring party is entitled to collect
monies that were accrued before the Retention End Date (if it
exists), or else the Agreement End Date but not yet distributed
by the societies. This date must be after the Retention End Date
(if it exists), or else the Agreement End Date. This date must be
entered if and only if the Post-term Collection Status is 'D'ate.
```
**Date of Signature
of Agreement**

92 8 D O The date when the written form of the agreement (the contract)
was signed.
**Number of Works** 100 5 N M Number of works registered subject to this agreement specific
to this file.

**Field** Start Size Fmt Req Field Description

**Sales/
Manufacture
Clause**

```
105 1 L C The S/M-clause-indicator is a marker that shows whether the
acquiring party has acquired rights either for products
manufactured or for products sold in the territories in
agreement. Synonym: Sales/Manufacture Clause. Only two
values are allowed according to BIEM/CISAC-rules:
```
S = Sales Clause: A stipulation which lays down that the acquiring
party has acquired rights for products sold in the territories in
agreement irrespective of the country of manufacture.

M = Manufacture Clause: A stipulation which lays down that the
acquiring party has acquired rights for products manufactured
in the territories in agreement irrespective of the country of sale.
This field is mandatory for specific agreements i.e. if Agreement
Type = OS or PS.

**Shares change** 106 1 B O If the shares for the writer interest can change as a result of sub-
publication or similar reason, set this field to “Y”. Default is “N”.

**Advance Given** 107 1 B O If there is an advance paid for this agreement, set this field to
“Y”. Default is “N”.


**_Version 2.1 field_**

```
Society-assigned
Agreement
Number
```
```
108 14 A O The agreement number assigned by the society of the sub-
publisher. This is generally not known when the AGR is
submitted but can be supplied by the societies in the ACK
transaction.
```
**Transaction Level Validation**

1. Only one AGR is allowed per transaction (TR).
2. There must be at least one TER record per AGR transaction (TR).
3. There must be at least two IPA records per AGR transaction -- one with an agreement role code of
   assignor; and one with an agreement role code of acquirer (TR).
4. The number of NWR/REV transactions in this file that contain reference to this agreement must be less
   than or equal the Number of Works. (TR)
5. The shares within the assignor and acquirer(s) records must total 100% for each type of right covered by
   the agreement within a tolerance of .06%. (TR)
6. The Transaction Type of the immediately preceding GRH must be AGR for submissions from a supplier.
   This edit is ignored if the AGR is part of an ACK (GR).

**Field Level Validation**

1. Submitter Agreement Number must be entered and must be unique within the submitter’s database.
   (TR)
2. The International Standard Agreement Code must, if entered, be a valid code. (FR - default to spaces)
3. Agreement Type must be entered and must be a valid entry in the Agreement Type Table. (TR)
4. The Agreement Start Date must be a valid date. (TR)
5. The Agreement Start Date must be entered for all agreement types. (TR)
6. If entered, the Agreement End Date must be a valid date. (TR)
7. If entered, the Retention End Date must be a valid date equal to or later than the Agreement End Date.
   (TR)
8. Prior Royalty Status must be entered, and must be “N” for none, or “A” for all, or “D” for date. (TR)
9. If the Prior Royalty Status is “D”, the Prior Royalty Start Date must be entered and must be earlier than
   the Agreement Start Date. (TR)
10. If the Prior Royalty Status is “N” or “A”, the Prior Royalty Start Date must not be entered. (TR)
11. If Prior Royalty Start Date is entered, it must be a valid date. (TR).
12. If Prior Royalty Start Date is entered, Prior Royalty Status must be “D”. (TR)
13. If entered, the Post-term Collection End Date must be a valid date and must be after the Retention End
    Date (if it exists), or else the Agreement End Date. (TR)
14. If Post-Term Collection Status is “D”, the Post-term Collection End-Date must be entered. (TR)
15. If Post-Term Collection Status is “N” or “O”, the Post-term Collection End-Date must not be entered. (TR)
16. Post-term Collection Status must be entered, and must be “N” for no or “O” for open-ended or “D” for
    date. (TR)


17. If Post-Term Collection End Date is entered, Post-term Collection Status must be “D”. (TR)
18. If entered, the Date of Signature of Agreement must be a valid date. (FR- default to zeros)
19. If entered, the Sales/Manufacture Clause must be “S” or “M”. (FR- default to space)
20. The Sales/Manufacture Clause must be entered for Agreement Type “OS” or “PS”. (TR)
21. Number of works must be numeric and greater than zero. (TR)
22. If Shares change is entered, it must be “Y” or “N”. (FR - default to “N”)
    _23._ If Advance Given is entered, it must be “Y” or “N”. (FR - default to “N”)
24. Edit has been removed.

### 4.2 NWR: New Work Registration

### 4.3 REV: Revised Registration

### 4.4 ISW: Notification of ISWC assign to a work

### 4.5 EXC: Existing work which is in Conflict with a Work Registration

**Transaction Description**

The New Works Registration (NWR) and Revised Registration (REV) are transactions that will be formatted
by publishers and sent to societies. The purpose of these transactions is to allow publishers to register and
revise works that they either own a percentage of or where they represent the owner as an administrator or
sub-publisher. Note that on revisions, all work information must be included in the transaction. Note also
that the NWR and REV transactions are also contained within an Acknowledgement (ACK) transaction sent
from the societies back to the submitting publisher.

The Notification of ISWC (ISW) will be sent from a society to a publisher to inform the publisher of the ISWC
that has been assigned to the musical work. When an ISWC is assigned to a work by a numbering agency,
duplicate checking takes place. It is possible that information originally sent on OWR, or OPU records has
changed as a result of another registration. Therefore publishers can update their databases with the
validated information in the ISW transactions.

The Details of Work in Conflict (EXC) will be sent from a society to a publisher to provide information on the
details of the work that is in conflict with the registration sent by the publisher. This transaction will follow
the Acknowledgement (ACK) transaction containing a status code of “CO” (for conflict). The EXC transaction
is only valid within an ACK, and following an NWR or REV transaction. Since the work described in the EXC
transaction may not have been registered via CWR, it may not meet the data standards outlined in this
document for the NWR/REV/ /ISW transactions.

A high priority flag has been introduced in version 2.1. It is meant to speed the registration of those works
that are on the charts, etc. It is expected that it will be used sparingly.

**Transaction Format**

```
Record
Type
```
```
Name Req Max
Use
```
```
Comments
```
```
NWR/
REV/
```
```
New Work Registration /
Revised Registration
```
```
M 1 NWR, REV are used to submit new or revised work
registrations to societies.
ISW/ Notification of ISWC
```

```
EXC Details of Work in Conflict ISW, EXC are used by societies to provide information back
regarding the registrations
SPU Publisher Controlled by
Submitter
```
```
C M List all publishers controlled by submitter. This record is
mandatory if writer ownership shares are less than 100%.
NPN Non-Roman Alphabet Publisher
Name
```
```
O M List the publisher name if available in a non-Roman
alphabet
SPT Publisher Territory of Control C M For each applicable SPU, list all territories where collection
rights exist.
OPT Publisher non-controlled
collection
```
```
O M For each applicable SPU, list territories where collection
rights do not exist
OPU Other Publisher O M
OPT Publisher Non-controlled
collection
```
```
O M For each OPU, list territories and collection
```
```
SWR Writer Controlled by Submitter O M List all the writers controlled by submitter.
```
**NWN** Non-Roman Alphabet Writer
Name

```
O M List the writer name if available in a non-Roman alphabet
```
**SWT** Writer Territory of Control C M For each applicable SWR, list all territories where
collection rights exist
**PWR** Publisher for Writer O M For each SWR, list the original publishers representing this
writer.
**OWR** Other Writer O M

```
PWR Publisher for Writer O M For each OWR, list the original publishers representing this
writer.
OWT Other writer collection O M For each applicable OWR, list all territories where non-
controlled collection rights exist
ALT Alternate Title O M
NAT Non-Roman Alphabet Title O M List the main title and any alternate titles available in non-
Roman alphabets
EWT Entire Work Title for Excerpts O 1
NET Non-Roman Alphabet Entire
Work Title for Excerpts
```
```
O 1
```
```
VER Original Work Title for Versions O 1
NVT Non-Roman Alphabet Original
Title for Version
```
```
O 1
```
```
PER Performing Artist O M
NPR Performing Artist in Non-Roman
alphabet
REC Recording O M
ORN Work Origin O M
INS Instrumentation Summary O M
IND Instrumentation Detail O M
COM Component O M
NCT Non-Roman Alphabet Title for
Components
```
```
O M
```
**NOW** Non-Roman Alphabet Other
Writer Name

```
O M The non-Roman alphabet name of a writer named in
Excerpt, Version, of Component record
ARI Additional Related Information O M Comments or Society work #
```
**XRF** (^) Work ID Cross Reference O M


**Record Description**

The NWR/REV//ISW/EXC record contains information specific to a work and occurring at a work level. Detail
records are listed subsequent to the NWR/REV//ISW/EXC record providing further information on the
content and ownership of the work.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = NWR (New Work Registration) for new
registrations, REV (Revised Registration) for revisions, or ISW
(Notification of ISWC) or EXC (Existing Work in Conflict) for
outgoing notifications.
Work Title 20 60 A M Name/Title of the work.
Language Code 80 2 L O The code representing the language of this title. These values
reside in the Language Code Table.
Submitter Work # 82 14 A M Number assigned to the work by the publisher submitting or
receiving the file. This number must be unique for the publisher.
ISWC 96 11 A O The International Standard Work Code assigned to this work.
Copyright Date 107 8 D O Original copyright date of the work.
Copyright
Number
```
```
115 12 A O Original copyright number of the work.
```
```
Musical Work
Distribution
Category
```
```
127 3 L M Describes the type of music as it applies to special distribution
rules. Values for this field reside in the Musical Work Distribution
Category Table.
Duration 130 6 T C Duration of the work in hours, minutes, and seconds. This field
must be greater than zero if Musical Work Distribution Category
is equal to SER. Note that some societies may also require
duration for works where the Musical Work Distribution
Category is equal to JAZ (e.g. BMI).
Recorded
Indicator
```
```
136 1 F M Indicates whether or not the work has ever been recorded.
```
```
Text Music
Relationship
```
```
137 3 L O Indicates whether this work contains music, text, and/or both.
Values reside in the Text Music Relationship Table.
Composite Type 140 3 L O If this is a composite work, this field will indicate the type of
composite. Values reside in the Composite Type Table.
Version Type 143 3 L M Indicates relationships between this work and other works. Note
that this field is used to indicate whether or not this work is an
arrangement. Values reside in the Version Type Table.
Excerpt Type 146 3 L O If this is an excerpt, this field will indicate the type of excerpt.
Values reside in the Excerpt Type Table.
Music
Arrangement
```
```
149 3 L C If Version Type is equal to “MOD”, this field indicates the type of
music arrangement. Values reside in the Music Arrangement
Table.
Lyric Adaptation 152 3 L C If Version Type is equal to “MOD”, this field indicates the type of
lyric adaptation. Values reside in the Lyric Adaptation Table.
Contact Name 155 30 A O The name of a business contact person at the organization that
originated this transaction.
Contact ID 185 10 A O An identifier associated with the contact person at the
organization that originated this transaction.
CWR Work Type 195 2 L O These values reside in the CWR Work Type table.
```

```
Grand Rights Ind 197 1 B C Indicates whether this work is originally intended for
performance on stage.
Note that this field is mandatory for registrations with the UK
societies.
Composite
Component Count
```
```
198 3 N C If Composite Type is entered, this field represents the number
of components contained in this composite.
Note that this is required for composite works where ASCAP is
represented on the work.
```
**_Society Specific Fields for Version 2.0_**

```
Date of
publication of
printed edition
```
```
201 8 D O For registrations with GEMA: Indicates the date that the printed,
new edition published by the submitting publisher appeared.
This information is especially relevant for the notification of sub-
published works by GEMA-sub-publishers.
Exceptional
Clause
```
```
209 1 F O For registrations with GEMA: By entering Y (Yes), the submitting
GEMA-sub-publisher declares that the exceptional clause of the
GEMA distribution rules with regard to printed editions applies
(GEMA-Verteilungsplan A Anhang III).
```
 **_Additional Fields for Version 2.0_**

```
Opus Number 210 25 A O Indicates the number assigned to this work, usually by the
composer. Part numbers are to be added with an # e.g. 28#3
(meaning Opus 28 part 3).
Catalogue
Number
```
```
235 25 A O The work catalogue number. The abbreviated name of the
catalogue is to be added (like BWV, KV), without dots. Part
numbers are to be added with an # e.g. KV 297#1 (meaning
Köchel Verzeichnis Nr.297 part 1).
```
 **_Fields for Version 2.1_**

```
Priority Flag 260 1 F O Indicates that this work should be processed faster because it is
on the charts, is by a well-known composer, etc.
```
**Transaction Level Validation**

1. Only one NWR or REV or ISW or EXC is allowed per transaction. (TR)
2. If Record Type is equal to REV and this work has not been successfully registered with the receiving
   society in a CWR transaction, then the record type will be changed to NWR. (FR)
3. Edit Replaced
4. Total Ownership shares across all SPU and OPU records must be less than or equal to 05000 (50%) for PR
   Shares and must be less than or equal to 10000 (100%) for MR Shares and SR Shares. Note that a
   tolerance of plus or minus 00006 (.06%) is allowed. (TR)
5. Edit has been removed_._
6. Edit has been removed
7. For each publisher controlled by the submitter that has collection shares, there must be at least one SPT
   record. (TR)
8. Detail records other than those listed in the Transaction Format table may not be submitted for this
   transaction (RR)


9. Total Ownership shares across all SWR and OWR records must be either 00000 (0%) or greater than or
   equal to 05000 (50%) for performing rights shares and less than or equal to 10000 (100%) for all rights.
   Note that a tolerance of plus or minus 00006 (.06%) is allowed. (TR)
10. If Version Type is equal to “MOD”, at least one SWR or OWR record must contain a Writer Designation
    Code equal to “AR” (Arranger), “AD”: (Adapter), “SR” (Sub-Arranger), “SA” (Sub-Author), or “TR”
    (Translator). (TR)

_Version 1.1 Edits_

11. The total ownership shares for all writers and publishers for each right must total either 10000 (100%)
    plus or minus 00006 (0.06%) or 00000 (0%). (TR)
12. A transaction must contain at least one writer record, SWR, or OWR. (TR)
13. If Record Type is "NWR", the work can not previously have been sent and accepted on a CWR transaction.
    (TR)

_Version 1.2 Edits_

14. The total of collection shares cannot exceed 100% for a given right for a territory. Note that a tolerance
    of plus or minus 00006 (.06%) is allowed. (TR)
15. There must be at least one writer (Writer Designation Code = “CA”, “A”, “C”) in a work. (TR)
16. A non-controlled publisher (OPU) can not appear in a chain started with a controlled original publisher
    (SPU). (TR)
17. There can only be one original publisher (Publisher Type = “E”) in a publisher chain. (TR)
18. Edit has been removed

_Version 2.0 Edits_

19. Edit has been removed.
20. For each writer controlled by the submitter that has collection shares, there must be at least one SWT.
    (TR)
21. Edit has been removed.
22. The Transaction Record Type (e.g. NWR or REV) must be the same as the Transaction Type of the
    immediately preceding GRH record. (GR)

_Version 2.1 Edits_

23. If Version Type is equal to “ORI”, there cannot be an SWR or OWR record that contains a Writer
    Designation Code equal to “AR” (Arranger), “AD”: (Adapter), “SR” (Sub-Arranger), “SA” (Sub-Author), or
    “TR” (Translator). (TR)
24. If all writers (SWR/OWR) are in the public domain, then the total ownership shares for performing rights
    must equal zero. (Since a PD work cannot be registered for performing rights). (TR)
25. For each SPU publisher chain (but not OPUs), the sum of at least one of PR Ownership Share, MR
    Ownership Share, or SR Ownership Share for SPU records with publisher type ‘E’ and ‘AQ’ must be greater
    than zero. (TR)
26. If an SPU record with publisher type “AQ” appears in a chain of title then the Ownership shares on the
    preceding Original publisher must be zero. (TR)


27. For performing rights: Within each chain of title, the sum of Collection Shares for any group of SPT records
    within a territory must equal the total of Ownership shares on the preceding Original Publisher and
    Acquirer SPU records provided:
    - The Shares Changed when sub-published is ‘N’
    - There is no interested party (SPU/POP/SWR/OWR) with a role code of ‘PA’ (income participant)

```
Note that a tolerance of plus or minus 00006 (.06%) is allowed. (TR)
```
28. Edit Removed
29. Edit Removed
30. For registrations to GEMA only - A work transaction must contain at least one SPU with either “PR
    Affiliation society” or “MR Affiliation society” set to 035 (GEMA), otherwise the work will be rejected
    with the transaction status set to “NP”. (TR – GEMA only)
31. For registrations to GEMA only – If all the Writer’s Last Names are entered as “DP” (i.e. Public Domain),
    then the work will be rejected with Transaction Status “NP”. (TR – GEMA only)
32. For registrations to Harry Fox only – A transaction must contain at least one SWR record. (TR – Harry Fox
    only)
33. Edit has been removed.
34. Edit has been removed
35. For registrations to SGAE only - All SPU records for Spanish sub-publishers must include a Society-
    assigned Agreement Number, or a Specific agreement must already have been notified to SGAE. (TR –
    SGAE only)
36. For Registrations to SGAE only - For SGAE territories, the total Ownership shares of non-controlled right-
    owners (OPUs and OWRs) and the Collection shares of controlled right-owners (SPTs and SWTs) must
    equal 100% for each right. Note that a tolerance of plus or minus 00006 (0.06%) is allowed. (TR – SGAE
    only)
37. For Registrations to SGAE only - For SGAE territories, the total of PR Ownership shares of Non-Controlled
    publishers (OPUs) and the PR Collection shares of controlled publishers (SPTs) must be less or equal to
    50%. Note that a tolerance of plus or minus 00006 (0.06%) is allowed. (TR – SGAE only)
38. For registrations to SGAE only – If all the Writers are Public Domain, then the work will be rejected. (TR
    - SGAE only)
39. If CWR Work Type is equal to “FM”, the transaction must include an ORN (Work Origin) record with a
    Production Title. (TR)
40. For registrations to ABRAMUS and UBC only – all SPU records for final Sub-Publisher’s collecting shares
    in the territory of Brazil must include the start date of the sub-publishing agreement in the Society-
    assigned Agreement Number field. The date should be left-justified in the format YYYYMMDD, and the
    rest of the field should be left blank. Invalid dates will be rejected. (TR – ABRAMUS and UBC only)
41. For registrations to SACEM only - A work transaction must contain at least one SPU with either “PR
    Affiliation society” or “MR Affiliation society” set to 058 (SACEM), otherwise the work will be rejected
    with the transaction status set to “NP”. (TR – SACEM only).
42. For registration SIAE only – There must be at least one writer designation code of C or CA on a transaction.
    (TR – SIAE only)

_Version 2.2 Edits_


43. The sequence of records within the transaction must be as follows: NWR/REV/ISW/EXC, SPU, NPN, SPT,
    OPT, OPU, NPN, OPT, SWR, NWN, SWT, PWR, OWR, NWN, OWT, PWR, ALT, NAT, EWT, NET, NOW, VER,
    NVT, NOW, PER, NPR, REC, ORN, INS, IND,COM, NCT, NOW, ARI, XRF (TR)
44. For any territory and any right type the total controlled collection (SPT/SWT)and non-controlled
    collection (OPT/OWT) must not be greater than 100. **Note** that a tolerance of plus 00006 (.06%) is
    allowed. (TR)
45. If the Text Music Relationship indicator is MUS (music only), then there should be no CA or A or SA or TR
    role codes among the work creators in SWR and OWR records (they should only be C or AR) (TR)
46. For registrations to ICE Societies only – If writer designation code AR on work, a different writer's
    designation code must be C or CA (TR – ICE only)
47. For registrations to ICE Societies only – If writer designation code SA on work, a different writer's
    designation code must be A or CA (TR – ICE only)
48. For registrations to MusicMark societies only. If SWR is BMI then any related publisher must be BMI or
    non-US PRO. If SWR is ASCAP then any related publisher must be ASCAP or non-US PRO (TR- MusicMark
    only)
49. For registrations to SGAE only. Part of the agreements hierarchy missed. There is no agreements link
    registered (notified) between original publisher and the submitter of the work (TR – SGAE only).
50. For registrations to SGAE only.The sub-publisher of the work is not the registered (notified) for the
    original publisher (TR – SGAE only).
51. For registrations to SGAE only.The shares for the sub-publisher are different from the shares registered
    (notified) in the agreement (TR – SGAE only).
52. For Registrations to ASCAP only: The sum of collection shares for the performing rights across all writers
    and publishers for the territory of US must equal 100% (plus or minus the usual tolerance). Note: if no
    OPT or OWT is present for the US, collection will be defaulted to ownership.
53. If Musical Work Distribution Category is equal to ‘SER’, the transaction must include an INS
    (Instrumentation Summary) record or an IND (Instrumentation Detail) record. (TR)

**Note:** If the transaction type is “EXC”, then all edits except the first may be bypassed since the existing work
that is in conflict may have been from a non-CWR registration.

**Field Level Validation**

1. Work Title must be entered. (TR)

_Revised in Version 1.2_

2. Language Code, if entered, must match an entry in the Language Code Table. (TR)
3. Submitter Work Number must be entered and must be unique for the party submitting the file. (TR)
4. If ISWC is entered, it must be a valid ISWC. (FR - default to spaces)
5. Copyright Date must be a valid date. (FR - default to zeros)
6. Musical Work Distribution Category must be entered and it must match an entry in the Musical Work
   Distribution Category table. (TR)


7. If Musical Work Distribution Category is equal to “SER”, Duration must be greater than zero and be a
   valid combination of hours, minutes and seconds. (TR)
8. If Music Work Distribution Category is not equal to “SER”, and Duration is entered, it must be a valid
   combination of hours, minutes, and seconds. (FR)
9. Recorded Indicator must be equal to “Y”, “N”, or “U”. (FR - default to “U”)
10. If Text Music Relationship is entered, it must match an entry in the Text Music Relationship table. (FR -
    default to spaces)
11. If Composite Type is entered, it must match an entry on the Composite Type table. (FR - default to spaces)
12. Version Type must be entered and must match an entry on the Version Type table. (TR)
13. If Excerpt Type is entered, it must match an entry on the Excerpt Type table. (FR - default to spaces)
14. If Version Type is equal to “MOD”, Music Arrangement must be entered and must match an entry in the
    Music Arrangement table. (TR)
15. If Version Type is equal to “MOD”, Lyric Adaptation must be entered and must match an entry in the Lyric
    Adaptation table. (TR)
16. If entered, Grand Rights Ind. must be equal to “Y” or “N”. (FR - default to spaces)
17. When entered, CWR Work Type must match an entry in the CWR Work Type table. (FR – default to
    spaces)
18. If Composite Type is entered, Composite Component Count must be entered. (TR)
19. If Composite Component Count is entered, Composite Type must be entered. (TR)
20. If entered, Composite Component Count must be numeric and must be greater than 1. (TR)
21. For registration BMI only - If Musical Work Distribution Category is equal to “JAZ” and BMI is represented
    on the work, duration must be greater than zero. (TR – BMI only)

_Version 1.1 Edits_

22. If entered, Music Arrangement must match an entry in the Music Arrangement table. (TR)
23. If entered, Lyric Adaptation must match an entry in the Lyric Adaptation table (TR).
24. Work Title must contain only valid ASCII characters from within the ‘Titles’ section of the allowed CIS
    character set. (TR)

_Version 2.0 Edits_

25. Edit has been removed
26. Edit Removed
27. Edit Removed
28. For registrations to GEMA only - If an ISWC number is entered and is part of the GEMA number block
    then that number must already be registered by GEMA. (FR – GEMA only – Replace with spaces)
29. For registrations to SACEM only - If an ISWC number is entered and is part of the SACEM number block
    then that number must already be registered by SACEM. (FR – SACEM only – Replace with spaces)

**Note:** If the transaction type is "EXC", then all edits regarding mandatory fields may be bypassed since the
existing work that is in conflict may have been from a non-CWR registration.


### 4.1 ACK: Acknowledgement of Transaction

**Transaction Description**

The ACK transaction allows for acknowledgements of transactions from a recipient back to a submitter. The
acknowledgement will indicate whether or not the recipient accepted the transaction. The ACK will include
any error or warning messages associated with the original transaction. In addition, the ACK includes the
NWR, or REV and possibly EXC transaction as the recipient processed it. The NWR/REV transaction will be as
the submitter sent it but supplemented with additional information such as IPI name numbers where
possible. In particular, the use of controlled/non-controlled record types will be as for the submitter. For
example, if a submitter sent a publisher on an SPU, the ACK will contain an SPU for that publisher. The ACK
transaction will contain all of the records sent by the submitter that have relevance to the recipient. For
example, a society will generally not return SPU/SPT records for sub-publishers in territories it does not
control.

MSG records will precede the detail record to which they apply and the sequence numbers in the Record
Prefix will contain information consistent to the ACK record - not the original transaction. Note that validation
will not stop at the first error encountered, but will continue to report all errors (unless a severe error makes
further processing inadvisable).

For version 2.2 there is no requirement to include the new 2.2 fields and record types in the ACK transaction.

**Transaction Format**

```
Record
Type
```
```
Name Req Max
Use
```
```
Comments
```
```
ACK Acknowledgement of
Transaction
```
```
M 1
```
```
MSG Message O M List all messages generated as a result of editing this
transaction.
AGR Agreement supporting Work
Registration
```
```
O 1 Detail records are included within the AGR transaction
```
```
NWR New Works Registration O 1 Detail records are included within the NWR transaction
REV Revised Registration O 1 Detail records are included within the REV transaction
EXC Existing Work in Conflict O 1 Detail records are included within the EXC transaction
```
**Record Description**

The ACK record identifies and provides a status on the transaction for which this acknowledgement has been
generated. In addition, other information is provided that can be used by the recipient to link the
acknowledgment back to the original transaction.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = ACK (Acknowledgement of Transaction)
Creation Date 20 8 D M The Creation Date of the original file that contained the
transaction to which this ACK applies.
Creation Time 28 6 T M The Creation Time of the original file that contained the
transaction to which this ACK applies.
Original Group ID 34 5 N M The Group ID within which the original transaction to which this
ACK applies.
```

```
Note that if the ACK is a result of a HDR or TRL record problem,
set this field to zeroes.
Original
Transaction
Sequence #
```
```
39 8 N M The Transaction Sequence # of the original transaction to which
this ACK applies.
Note that if the ACK is a result of a HDR or TRL record problem,
set this field to zeroes.
Field Start Size Fmt Req Field Description
Original
Transaction Type
```
```
47 3 L M The Transaction Type of the original transaction to which this
ACK applies.
Note that if the ACK is a result of a HDR or TRL record problem,
set this field to HDR or TRL (whichever is applicable).
Creation Title 50 60 A C The creation title as delivered by the submitter (i.e. the title of
the musical work or audio visual production...). This field is
required if the ACK is in response to an NWR or REV transaction.
```
```
Submitter
Creation #
```
```
110 20 A C The unique identifier assigned by the original submitter to this
work. This field is required if the ACK is in response to a
transaction.
Recipient
Creation #
```
```
130 20 A C The unique identifier assigned by the recipient to this work. This
field is required if the ACK is in response to a transaction and if
the transaction status indicates that the recipient has accepted
the work.
Processing Date 150 8 D M The date this transaction or file was formally processed by the
recipient.
Transaction
Status
```
```
158 2 L M The current status of this transaction. Values for this field reside
in the Transaction Status Table.
```
**Transaction Level Validation**

1. Only one ACK is allowed per transaction. (TR)

_Version 2.0 Edits_

2. The ACK transaction must be followed by one of NWR, REV, or AGR transactions. (TR)
3. The EXC transaction must follow an NWR or REV transaction within the ACK transaction. (TR)
4. The Transaction Type of the immediately preceding GRH must be ACK for submissions from a society.
   (GR)

**Field Level Validation**

1. The combination of Creation Date and Creation Time must match the same fields found on the HDR
   record of a file generated by the submitter. (TR)
2. The combination of Original Group Number and Transaction Sequence # must be valid within the file
   referred to by Creation Date and Creation Time. (TR)
3. Original Transaction Type must match the transaction referred to by the combination of Creation Date,
   Creation Time, Original Group #, and Original Transaction Sequence #. (TR)
4. Creation Title must match the corresponding title associated with the Submitter Creation #. (TR)
5. Submitter Creation # must match an entry in the submitter’s data base. (TR)
6. If entered, Recipient Creation # must match an entry in the recipient’s data base. (TR)


7. Processing Date must be a valid date. (TR)
8. Transaction Status must match an entry in the Transaction Status table. (TR)

_Version 2.0 Edits_

9. Creation Title is required if the ACK is in response to an NWR or REV transaction. (TR)

## 5 Detail Records

Detail Records contain the information required to define each transaction. The following record types are
defined in this section...

```
 TER: Territory in Agreement
```
```
 IPA: Interested Party of Agreement
```
```
 SPU: Publisher Controlled by Submitter
```
```
 NPN: Non-Roman Alphabet Publisher Name
```
```
 SPT: Publisher Territory of Control
```
```
 OPT: Publisher non-controlled collection
```
```
 OPU: Other Publisher
```
```
 SWR: Writer Controlled by Submitter
```
```
 NWN: Non-Roman Alphabet Writer Name
```
```
 SWT: Writer Territory of Control
```
```
 OWT: Other writer collection
```
```
 PWR: Publisher for Writer
```
```
 OWR: Other Writer
```
```
 ALT: Alternate Title
```
```
 NAT: Non-Roman Alphabet Title
```
```
 EWT: Entire Work Title for Excerpts
```
```
 NET: Non-Roman Alphabet Entire Work Title for Excerpts
```
```
 VER: Original Work Title for Versions
```
```
 NVT: Non-Roman Alphabet Original Title for Versions
```
```
 PER: Performing Artist
```

```
 NPR: Performing Artist Name in Non-Roman Alphabet
```
```
 REC: Recording Detail
```
```
 ORN: Work Origin
```
```
 INS: Instrumentation Summary
```
```
 IND: Instrumentation Detail
```
```
 COM: Composite Component
```
```
 NCT : Non-Roman Alphabet Title for Components
```
```
 NOW: Non-Roman Alphabet Other Writer Name
```
```
 MSG: Message
```
```
 ARI: Additional Related Information
```
```
 XRF: Work ID cross reference
```
### 5.1 TER: Territory in Agreement

**Record Description**

The TER record specifies a territory either within the territorial scope of the preceding AGR agreement or
excluded from it. An agreement may cover several territories. Include one TER record for each territory, or
groups of territories covered by the agreement. It is also possible to use a combination of include and exclude
TER records. For example, if an agreement applied to all of Europe except Switzerland, you can provide a TER
record to include Europe, and one to exclude Switzerland.

The TER record must follow the AGR record to which it applies or other TER records for the same AGR
agreement.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = TER (Territory in Agreement)
Inclusion/
Exclusion
Indicator
```
```
20 1 L M This is a marker which shows whether the territory specified in
this record is part of the territorial scope of the agreement or
not. Possible entries are I (= territory included) and E (= territory
excluded).
TIS Numeric Code 21 4 L M Numeric identifier of a territory according to the new CISAC
Territory Standard.
```
**Record Level Validation**

1. Must follow an AGR or TER record. (TR)


**Field Level Validation**

1. The Inclusion/Exclusion Indicator must be “I” or “E”. (TR)
2. The TIS Numeric Code must match an entry in the _TIS._ (TR)

### 5.2 IPA: Interested Party of Agreement

**Record Description**

The IPA record contains information on the interested parties that concluded the agreement and on the
shares they have agreed to assign through the agreement. Each AGR record must be followed by at least one
assignor IPA record and at least one acquirer IPA record.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = IPA (Interested Party of Agreement)
Agreement Role
Code
```
```
20 2 L M Code defining the role of the interested party in this agreement.
The interested party can be either an assigning party or an
acquiring party.
Interested Party
IPI Name #
```
```
22 11 L O The IPI name # assigned to this interested party. These values
reside in the IPI Database.
IPI Base Number 33 13 L O The IPI base number assigned to this interested party.
Interested
Party #
```
```
46 9 A M Submitter’s unique identifier for this interested party.
```
```
Interested Party
Last Name
```
```
55 45 A M The last name of this writer or the name of the publisher. Note
that if the submitter does not have the ability to split first and
last names of writers, the entire name should be entered in this
field in the format “Last Name, First Name” including the comma
after the last name.
Interested
Party Writer First
Name
```
```
100 30 A O The first name of this writer along with all qualifying and middle
names. An entry is only allowed if the Agreement type is OS or
OG and the Agreement Role Code is Assignor.
PR Affiliation
Society
```
```
130 3 L C Number assigned to the performing rights society with which
the IP is affiliated. These values reside on the Society Code Table.
Required if PR share is greater than zero.
PR Share 133 5 N
999v99
```
```
C Defines the percentage of the performing rights to the work,
claimed by this interested party. Within an individual IPA record,
this value can range from 0 to 100%. The PR-shares of all IPA
records within an AGR transaction must add up to 100.00 if the
agreement covers performing rights or must be 00000 if the
agreement does not cover performing rights. At least one of PR
share, MR share, or SR share must be greater than 0.
```
```
MR Affiliation
Society
```
```
138 3 L C Number assigned to the mechanical rights society with which
this interested party is affiliated. These values reside on the
Society Code Table. Required if MR share is greater than zero.
```

```
MR Share 141 5 N
999v99
```
```
C Defines the percentage of the mechanical rights to the work,
claimed by this interested party. Within an individual IPA record,
this value can range from 0 to 100.00. The MR-shares of all IPA
records within an AGR transaction must add up to 100.00 if the
agreement covers the mechanical rights or to 000.00 if the
agreement does not cover the mechanical rights. At least one of
PR share, MR share, or SR share must be greater than 0.
SR Affiliation
Society
```
```
146 3 L C Number assigned to the synchronization rights society with
which the IP is affiliated. These values reside on the Society Code
Table. Required if SR share is greater than zero.
```
```
Field Start Size Fmt Req Field Description
SR Share 149 5 N
999v99
```
```
C Defines the percentage of the synchronization rights to the
work, claimed by this interested party. Within an individual IPA
record, this value can range from 0 to 100.00. The SR-shares of
all IPA records within an AGR transaction must add up to 100.00
if the agreement covers the synchronization rights or to 000.00
if the agreement does not cover the synchronization rights. At
least one of PR share, MR share, or SR share must be greater
than 0.
```
**Record Level Validation**

1. Must follow a TER or IPA record. (TR)
2. Each AGR record must be followed by one assignor IPA record and at least one acquirer IPA record. (TR)
3. At least one of PR share, MR share, or SR share must be greater than 0 in an acquirer IPA record. (TR)

**Field Level Validation**

1. Agreement Role Code must be entered and must be either “AS” for assignor or “AC” for acquirer. (TR)
2. If entered, Interested Party IPI Name # must match an entry in the IPI system. (FR)
3. Interested Party # cannot duplicate the number of a different interested party currently or previously
   controlled by the submitting publisher. (TR)
4. The Interested Party Writer First Name entry is only allowed if the Agreement type is “OS” or “OG” and
   the Agreement Role Code is Assignor. (FR)
5. If the PR Affiliation Society is entered, it must be held in the Society Code Table. (FR)
6. PR Share must be numeric and in the range 00000 (0%) to 10000 (100%). (TR)
7. If the MR Affiliation Society is entered, it must reside in the Society Code Table. (FR)
8. The MR Share must be numeric and in the range 00000 (0%) to 10000 (100%). (TR)
9. If the SR Affiliation Society is present, it must reside in the Society Code Table. (FR)
10. The SR Share must be numeric and in the range 00000 (0%) to 10000 (100%). (TR)
11. One of PR Affiliation Society or MR Affiliation Society must be entered. (TR)
12. If PR Share is entered, then PR Affiliation must be entered. (TR)
13. If MR Share is entered, then MR Affiliation must be entered. (TR)
14. If SR Share is entered, then SR Affiliation must be entered. (TR)


15. Interested Party Last Name must be entered. (TR)
16. Interested Party # must be entered (TR)
17. If IPI Base Number is entered, it must match an entry in the IPI database. (FR)
18. Interested Party Last Name must contain only valid ASCII characters from within the “Names” section of
    the allowed CIS character set. (TR)
19. If entered, Interested Party Writer First Name must contain only valid ASCII characters from within the
    “Names” section of the allowed CIS character set. (TR)

### 5.3 NPA: Non-Roman Alphabet Agreement Party Name

**Record Description**

This record identifies names in a non-roman alphabet for the acquiring parties of this agreement. The
language code is used to identify the alphabet. This record can be used to identify the name of the party in
the preceding IPA record.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = NPA
Interested
Party #
```
```
20 9 A C Submitting publisher’s unique identifier for this Writer.
```
```
Interested
Party Name
```
```
29 160 A M The last of a writer or the publisher name.
```
```
Interested
Party Writer First
Name
```
```
189 160 A M The first name of a writer.
```
```
Language Code 349 2 L O The Language code of the name – must be a valid code from the
Language Code Table.
```
**Record Level Validation**

1. This record must immediately follow an IPA that refers to the interested party named here. (RR)

**Field Level Validation**

1. Interested Party # must be entered and must match the Interested Party # of the corresponding IPA
   record. (RR)
2. Interested Party Name must be entered. (RR)
3. Language code, if entered, must be a valid code from the Language Code Table. (FR)

### 5.4 SPU: Publisher Controlled By Submitter

### 5.5 OPU: Other Publisher

**Record Description**

The SPU record contains information about original publishers, income participants, sub-publishers, and/or
administrators who are involved in the ownership and collection of this work and are under the control of
the party submitting this transaction. The OPU record contains information about original publishers that are
not under the control of the party submitting this transaction.


**Submission of SPU Records**

A transaction must include one or more publisher chains. The first SPU record within a chain must represent
an original publisher or income participant (Publisher Type = E or PA). That record may be followed by one
SPU record that indicates the original publisher’s local administrator if one exists (Publisher Type = AM) and
the collection shares for the administrator. Subsequent to the administrator record, SPU records
representing sub-publishers (Publisher Type = SE) can be inserted. Also note that subsequent to each SPU
record, SPT records (Publisher Territory of Control) can be inserted to designate the territory of the collection
shares for the previous publisher.

OPT (publisher non-controlled collection) records can be included to give a complete picture of collection for
a territory. These follow the relevant SPU/OPU. If there is a mix of controlled and non-controlled territories
for an SPU the OPT(s) follow the SPT(s).

Income participants may start a publisher chain, or be included in one.

The following chart demonstrates the hierarchy that exists between these records (note that the shaded box
indicates a record that can repeat).

```
SPT for each
territory controlled
by the original
publisher
```
```
SPT for each
territory
controlled by the
sub publisher
```
```
SPU Record for
Original
Publisher
```
```
SPU for the
administrator
```
```
SPT for the
territory
controlled by the
administrator
```
```
SPU for each
sub publisher
```
```
SPU for the
administrator
```
```
SPT for the
territory
controlled by the
administrator
```

Note that the hierarchy repeats for each original publisher represented on the work. The following list is a
representation of the hierarchy based on the appearance of records within a transaction:

```
SPU (Original Publisher/Income Participant)
```
```
SPT (Territory of Control) for Original Publisher/Income Participant
```
- _Repeat SPT for each territory included or excluded_
-
-

```
SPU (Administrator Publisher for previous Original Publisher/Income
Participant)
```
```
SPT (Territory of Control) for Administrator
```
- _Repeat SPT for each territory included or excluded_
-

```
SPU (First Sub-Publisher for previous Original Publisher/Income Participant)
```
```
SPT (First Territory of Control) for Sub-Publisher
```
- _Repeat SPT for each territory included or excluded applying_

```
to this Sub-Publisher that is not locally administered
```
-

```
SPU (Administrator Publisher) for previous Sub-Publisher if required
```
```
SPT (Territory of Control) for Administrator
```
- _Repeat SPT for each territory applying to this administrator_
-
-

```
SPU (Last Sub-Publisher for previous Original Publisher/Income Participant)
```
```
SPT (First Territory of Control) for Sub-Publisher
```
- _Repeat SPT for each territory included or excluded_
-
-

```
SPU (Administrator Publisher) for previous Sub-Publisher if required
```
```
SPT (Territory of Control) for Administrator
```
- _Repeat SPT for each territory applying to this administrator_
-

All SPU records with the same Publisher Sequence Number as the Original Publisher/Income Participant SPU
are considered to be “linked” to the Original Publisher/Income Participant in a chain. Note that this hierarchy


can be repeated if there are multiple Original Publishers for this work represented by the submitter of the
file.

On co-publishing/administration deals, note that the co-publisher/administrator needs to submit two SPU
records for itself - one as co-publisher and one as administrator for the original publisher they are
administering.

The publisher to publisher agreement numbers are recorded in the SPU for the sub-publisher or
administrator. It is the society of the sub-publisher or the acquiring party that assigns the society-assigned
agreement number to the publisher to publisher agreement. If submitted, the AGR contains a Society
Agreement Number that is used to link the agreement to a work registration.

**Submission of OPU Records**

When submitting OPU records, the first record in the chain must be the original publisher. If you know the
sub-publisher(s) or administrator(s), these may be included in the chain if you choose. Publisher non-
controlled collection (OPT) records are optional in a publisher chain with an OPU.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = SPU (Publisher Controlled by Submitter) or
OPU (Other Publisher)
Publisher
Sequence #
```
```
20 2 N M A sequential number assigned to the original publishers on this
work.
Interested
Party #
```
```
22 9 A C Submitting publisher’s unique identifier for this publisher. This
field is required for record type SPU and optional for record type
OPU.
Publisher Name 31 45 A C The name of this publishing company. This field is required for
record type SPU and optional for record type OPU.
Publisher
Unknown
Indicator
```
```
76 1 F C Indicates if the name of this publisher is unknown.
Note that this field must be left blank for SPU records. For OPU
records, this field must be set to “Y” if the Publisher Name is
blank.
Publisher Type 77 2 L C Code defining this publisher’s role in the publishing of the work.
These values reside on the Publisher Type Table. This field is
required for record type SPU and optional for record type OPU.
Tax ID # 79 9 A O The number used to identify this publisher for domestic tax
reporting.
Publisher
IPI Name #
```
```
88 11 L C The IPI Name # assigned to this publisher. If the record is of type
SPU and followed by an SPT (and hence represents the file
submitter), then the IPI Name Number is mandatory.
Submitter
Agreement
Number
```
```
99 14 A O Indicates the agreement number unique to the submitter under
which this publisher has acquired the rights to this work.
```
```
PR Affiliation
Society #
```
```
113 3 L C Number assigned to the Performing Rights Society with which
the publisher is affiliated. These values reside on the Society
Code Table.
PR Ownership
Share
```
```
116 5 N
999v99
```
```
C Defines the percentage of the publisher’s ownership of the
performance rights to the work. This share does not define the
percentage of the total royalty distributed for performance of
the work that will be collected by the publisher. Within an
individual SPU record, this value can range from 0 to 50.0.
```

```
MR Society 121 3 L C Number assigned to the Mechanical Rights Society with which
the publisher is affiliated. These values reside on the Society
Code Table.
MR Ownership
Share
```
```
124 5 N
999v99
```
```
C Defines the percentage of the publisher’s ownership of the
mechanical rights to the work. This share does not define the
percentage of the total royalty distributed for sales of CDs,
Cassettes, etc. containing the work that will be collected by the
publisher. Within an individual SPU record, this value can range
from 0 to 100.0.
SR Society 129 3 L C Number assigned to the Society with which the publisher is
affiliated for administration of synchronization rights. These
values reside on the Society Code Table.
SR Ownership
Share
```
```
132 5 N
999v99
```
```
C Defines the percentage of the publisher’s ownership of the
synch rights to the work. This share does not define the
percentage of the total money distributed that will be collected
by the publisher. Within an individual SPU record, this value can
range from 0 to 100.0.
```
```
Field Start Size Fmt Req Field Description
Special
Agreements
Indicator
```
```
137 1 L O Indicates publisher claiming reversionary rights. Note that this
flag only applies to societies that recognize reversionary rights
(for example, SOCAN).
First Recording
Refusal Ind
```
```
138 1 F O Indicates whether the submitter has refused to give authority
for the first recording.
Note that this field is mandatory for registrations with the UK
societies.
Filler 139 1 A O Fill with a blank
```
 **_Version 2.0 Fields_**

```
Publisher IPI Base
Number
```
```
140 13 L O The IPI base number assigned to this publisher
```
```
International
Standard
Agreement Code
```
```
153 14 A O The ISAC that has been assigned to the agreement under which
this publisher share is to be administered.
```
```
Society-assigned
Agreement
Number
```
```
167 14 A O The agreement number assigned by the society of the sub-
publisher.
```
 **_Version 2.1 Fields_**

```
Agreement Type 181 2 L O Code defining the category of agreement. The values reside in
the Agreement Type Table.
USA License Ind 183 1 L O Indicates that rights flow through SESAC/BMI/ASCAP/AMRA in
the US
```
**Record Level Validation**

1. Edit has been removed.
2. The first SPU record within a chain must be for an Original Publisher or Income Participant (Publisher
   Type = “E” or “PA”). (TR)
3. If Publisher Type is equal to “SE” or “AM” or “PA” or “ES”, Ownership Shares must be equal to zero.
   (TR)


_Version 1.1 Edits_

4. If Publisher Type is equal "AM", the publisher must have the right to administer for the preceding
   publisher. (TR)

_Version 1.2 Edits_

5. Administrators and sub-publishers must be assigned the publisher sequence number belonging to
   the original publisher or income participant on whose behalf they administer/sub-publish. (TR)

_Version 2.0 Edits_

6. Edit has been removed.
    7. Edit has been removed.

_Version 2.1 Edits_

8. If the record represents an Acquirer, at least one of PR Ownership share, MR Ownership share, or SR
   Ownership share must be greater than zero. (TR)
9. For Registrations to ASCAP only – Where there is an ASCAP Original publisher there must be a
   collection territory that includes the United States within that chain of title. (TR – ASCAP only)
10. Edit has been removed.
11. For registrations to SGAE only - SPU records for final Sub-Publishers with SGAE interest must include a
    pre-notified Society-assigned Agreement number unless the Agreement Type field is set to “OS” or “PS”.
    (TR – SGAE only)
12. For registrations to SGAE only - If the record is of SGAE interest then it must include an Agreement Type.
    (TR – SGAE only)
13. For registration MusicMark only – If the original publisher belongs to ASCAP then the sub-publisher or
    administrator in the chain for the US cannot belong to BMI or SESAC. (TR – MusicMark only)
14. For registration MusicMark only – If the original publisher belongs to BMI or SESAC then the sub-publisher
    or administrator in the chain for the US cannot belong to ASCAP. (TR – MusicMark only)

**Field Level Validation**

1. Publisher Sequence # must be entered. The first publisher chain on a work must have Publisher Sequence
   # set to 1. Each subsequent publisher chain must be set to the Publisher Sequence # of the prior publisher
   chain incremented by 1. (TR)
2. If Record Type is equal to SPU, Interested Party # must be entered. (TR)
3. Submitters must ensure that the Interested Party # is unique within their system for both current and
   past controlled parties. (TR)
4. If Record Type is equal to SPU or Publisher Unknown Indicator is not equal to “Y”, Publisher Name must
   be entered. (TR)
5. If Record Type is equal to SPU, Publisher Type must be entered. (TR)
6. If Publisher Type is entered in an SPU record, it must match an entry in the Publisher Type table. (TR)
7. If Record Type is equal to SPU, Publisher Unknown Indicator must be blank. (TR)
8. If Record Type is equal to OPU and Publisher Unknown Indicator is entered, it must be equal to “Y” or
   “N” (FR - default to “N”)


9. If Record Type is equal to OPU and Publisher Unknown Indicator is equal to “Y”, Publisher Name must be
   blank. (FR - default Publisher Unknown Indicator to “N”)
10. If Publisher IPI Name # is entered, it must match a publisher entry in the IPI database. (FR – default to
    blank)
11. If Submitter Agreement Number is entered, it must match the identifier for an agreement on file with
    the society of the acquiring party or it must match an identifier in the AGR transaction. (FR - default to
    spaces)
12. If entered, PR Affiliation Society # must match an entry in the Society Code table. (FR – default to spaces)
13. PR Ownership Share must be numeric. The value must also be between 00000 (0%) and 05000 (50.00%).
    Note that the decimal point on this field is implied and should not be present within the field. (TR)
14. Edit has been removed.
15. If entered, MR Affiliation Society # must match an entry in the Society Code table (FR – default to spaces)
16. MR Ownership Share must be numeric. The value must also be between 00000 (0%) and 10000
    (100.00%). Note that the decimal point on this field is implied and should not be present within the field.
    (TR)
17. If entered, SR Affiliation Society # must match an entry in the Society Code table. (FR – default to spaces)
18. SR Ownership Share must be numeric. The value must also be between 00000 (0%) and 10000 (100.00%).
    Note that the decimal point on this field is implied and should not be present within the field. (TR)
19. All ownership shares must be equal to 0 if Publisher Type is not equal to “E” or “AQ” (i.e. only Original
    Publishers can have ownership shares greater than zero). (TR)
20. If entered, Special Agreement Indicator must match an entry in the Special Agreement Indicator table.
    (FR - default to spaces)
21. If entered, First Recording Refusal Ind must be equal to Y or N. (FR - default to spaces)
22. Edit has been removed.

_Version 1.1 Edits_

23. If entered, Tax ID must be numeric. (FR - default to zeros)

_Version 2.0 Edits_

24. If Publisher IPI Base Number is entered, it must match an entry in the IPI database. (FR)
25. If International Standard Agreement Code is entered, it must match an entry in the international
    agreements database. (FR - default to spaces).
26. If Society-Assigned Agreement Number is entered, it must match the identifier for an agreement on file
    with the society of the acquiring publisher. (FR - default to spaces)
27. If Record Type is “OPU”, Special Agreements Indicator can only be “L” or blank. (FR - default to space)
28. If Record type is “OPU”, and Publisher type is invalid or missing, default to “E”. (FR - default to “E”)
29. If the Publisher Name matches the name of a society in the Society Code table, and the Publisher IPI
    Name # is missing or invalid, then the registration is invalid. (TR)

_Version 2.1 Edits_

30. If Agreement Type is entered, it must match an entry in the Agreement Type table. (FR)


31. If USA License Ind is entered, it must match a value in the USA License Indicator table. (FR)
32. If the role code is ‘AQ’, this SPU record must follow an SPU record with a role code of ‘E’. (TR)
33. For registrations to GEMA only - If Agreement Type is equal to “PS”, the Submitter Agreement
    Number must be entered and must match an entry in a corresponding AGR-record. (TR - GEMA only)
34. For registrations to GEMA only – If Record Type is equal to “SPU” and Publisher Type is equal to “SE”, an
    Agreement Type must be entered. (TR – GEMA only).
35. Edit has been removed.
36. Edit has been replaced
37. If entered, Publisher Name must contain only valid ASCII characters from within the “Names” section of
    the allowed CIS character set (TR)
38. For registrations to SACEM only – If Record Type is equal to “SPU” and Publisher Type is equal to “SE”,
    an Agreement Type must be entered. All SPU records for final Sub-Publishers in the FR must supply pre-
    notified Society-assigned Agreement Number. (TR – SACEM only).
39. Edit Replaced (see 46)

_Version 2.2 Edits_

40. If Record Type is equal to SPU and is the collecting publisher the Publisher IPI Name Number must be
    entered. (TR)
41. For registrations to Societies requiring Society Assigned Agreement Numbers only: A Society Assigned
    Agreement Number must be provided missing on SPU (TR)
42. For registrations to Societies requiring Society Assigned Agreement Numbers only:– The provided Society
    Assigned Agreement Number must be not represent a terminated agreement (TR)
43. For registrations to Societies requiring Society Assigned Agreement Numbers only: The provided Society
    Assigned Agreement Number must exist in the Society system (TR)
44. For registrations to Societies requiring Society Assigned Agreement Numbers only:– The claimed territory
    on the provided Society Assigned Agreement Number must be included in the agreement on file at the
    Society (TR)
45. For registrations to ICE Societies only – The Society Assigned Agreement Number must refer to the
    quoted combination of Submitter, Assignor and Agreement number on SPU on transaction (TR – ICE only)
    For registrations to SACEM only: The IPI-name-number provided for the Original Publisher SPU must be
    equal to the assignor IPI-name-number for the agreement represented by the first 7 characters of the
    Society Provided Agreement Number.
46. For registrations to SACEM only: The IPI-name-number provided for the Sub-Publisher SPU must be equal
    to the assignee IPI-name-number for the agreement represented by the last 7 characters of the Society
    Provided Agreement Number.

### 5.6 NPN: Non-Roman Alphabet Publisher Name

**Record Description**

This record identifies publisher names in non-roman alphabets for this work. The language code is used to
identify the alphabet. This record can be used to identify the name of the publisher in the preceding SPU/OPU
record.


**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type =NPN
Publisher
Sequence #
```
```
20 2 N M A sequential number assigned to the original publishers on this
work.
Interested
Party #
```
```
22 9 A M Submitting publisher’s unique identifier for this publisher.
```
**Publisher Name** 31 480 A M The name of this publishing company in non-roman alphabet.
**Language Code** 511 2 L O The Language code of the name – must be a valid code from the
Language Code Table.
**Record Level Validation**

1. This record must immediately follow an SPU that refers to the publisher named here. (RR)

**Field Level Validation**

1. Publisher Sequence # must be entered and must match the Publisher Sequence # of the
   corresponding SPU record. (RR)
2. Interested Party # must be entered and must match the Interested Party # of the corresponding SPU
   record. (RR)
3. Publisher Name must be entered. (RR)
4. Language code, if entered, must be a valid code from the Language Code Table. (FR)

### 5.7 SPT: Publisher Territory of Control

### 5.8 OPT: Publisher Non-Controlled Collection

**Record Description**

The SPT record defines the territory and the collection shares for the preceding SPU publisher. Note that SPT
records follow an SPU, NPN (Non-Roman alphabet Publisher Name), or another SPT record. The SPT record
cannot be used with OPU records. Include one SPT record for each territory, or groups of territories for which
the preceding publisher has collection rights. It is also possible to use a combination of include and exclude
SPT records. For example, to specify collection shares for all of Europe except Switzerland, you can provide
an SPT record to include Europe, and one to exclude Switzerland. By its nature, the SPT used to exclude a
territory should not have any share percentages greater than zero. It is possible to have all zero shares on an
SPT that includes one or more territories. Such a record would be used to record a publisher’s place in the
chain of agreements.

The Shares change flag alerts the recipient that the ownership will not equal collection for this publisher
chain. Usually this is because the agreements permit shares to change when the work is sub-published. The
sequence number which was added in version 2.1 should run from 1 to the number of SPTs for each SPU.

The OPT record is used to record non-controlled collection. OPT records are optional and should not be
treated as providing authoritative information (whether it follows an OPU or an SPU). They can be useful in
making clear the total collection for a territory where shares change on sub-publication or where the
territories of controlled collection vary between chains.

An OPT can follow an OPU or an SPU. An SPT can never follow an OPU in a non-controlled chain. An OPT can
follow an SPT under the same SPU and an SPT can follow an OPT under the same SPU.


**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = SPT (Publisher Territory of Control) or OPT
(publisher non-controlled collection)
Interested
Party #
```
```
20 9 A M Submitting publisher’s unique identifier for this Publisher.
```
```
Constant 29 6 A M Set this field equal to spaces.
PR Collection
Share
```
```
35 5 N
999v99
```
```
C Defines the percentage of the total royalty distributed for
performance of the work which will be collected by (paid to) the
publisher within the above Territory. It can be a range from 0 to
50.00.
MR Collection
Share
```
```
40 5 N
999v99
```
```
C Defines the percentage of the total royalty distributed for sales
of CDs, Cassette Tapes, etc. in which the work is included which
will be collected by (paid to) the publisher. It can be a range from
0 to 100.00.
SR Collection
Share
```
```
45 5 N
999v99
```
```
C Defines the percentage of the total royalty distributed for
Synchronization rights to the work which will be collected by
(paid to) the publisher. It can be a range from 0 to 100.00.
```
 **_Version 2.0 Fields_**

```
Inclusion/
Exclusion
Indicator
```
```
50 1 L M This is a marker which shows whether the territory specified in
this record is part of the territorial scope of the agreement or
not. Possible entries are I (= territory included) and E (= territory
excluded).
TIS Numeric Code 51 4 L M A territory within which this publisher claims the right to collect
payment for performance or use of this work. These values
reside in the TIS Code Table.
Shares change 55 1 B O If the shares for the writer interest change as a result of sub-
publication in this territory or for a similar reason, set this field
to “Y”.
```
 **_Version 2.1 Fields_**

```
Sequence # 56 3 N M A sequential number assigned to each territory following an
SPU.
```
**Record Level Validation**

1. If the Inclusion/Exclusion Indicator is “I”, at least one of PR Collection Share, MR Collection Share, or SR
   Collection Share must be greater than zero. (TR)
2. Edit has been removed.
3. Edit has been removed.
4. Edit has been removed.

_Version 2.0 Edits_

5. Each Territory (TIS code) included on an SPT/OPT record can only be linked to one SPU/OPU for a given
   right. (TR)
6. For registrations to ASCAP only – The first SPT record immediately following an SPU must have the
   “Inclusion/Exclusion Indicator” set to “I”. (TR – ASCAP only)


**Field Level Validation**

1. When entered, SPT records must follow an SPU, NPN or SPT record. (TR)
2. The Interested Party # must be entered and must be equal to the Interested Party # on the previous SPU
   record. (TR)
3. Edit has been removed.
4. Edit has been removed.
5. Edit has been removed.
6. PR Collection Share must be between 00000 (0%) and 5000 (50%). (TR)
7. MR Collection Share must be between 00000 (0%) and 10000 (100%). (TR)
8. SR Collection Share must be between 00000 (0%) and 10000 (100%). (TR)

_Version 2.0 Edits_

9. TIS Numeric Code must be entered and must match an entry in the TIS database. (TR)
10. Inclusion/Exclusion Indicator must be entered and must be either “E” for excluded or “I” for included.
    (TR)
11. If Shares change is entered, it must be set to “Y” or “N”. (FR - default – “N”)

_Version 2.1 Edits_

12. Sequence # must be present. (RR)
13. Sequence # must be 1 for the first SPT/OPT after an SPU/OPU, and increment by 1 for each subsequent
    SPT. (RR)

_Version 2.2 Edits_

14. When entered, OPT records must follow an SPU, NPN, SPT, OPU or OPT record. (TR)

### 5.9 SWR: Writer Controlled By Submitter

### 5.10 OWR: Other Writer

**Record Description**

The SWR record contains specific information on a writer controlled by the submitting publisher. Submitters
will, on a best efforts basis, provide either the writer’s tax id number (e.g. Social Security Number) or their
IPI Name # to ensure exact identification by representative societies.

The OWR record contains whatever information is available on writers that are not controlled by the
submitting publisher.

Subsequent to each SWR record, SWT records (Writer Territory of Control) must be inserted to designate
Collection Shares for the related writer within a designated territory.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = SWR (Writer Controlled by Submitter) or OWR
(Other Writer)
```

```
Interested
Party #
```
```
20 9 A C Submitting publisher’s unique identifier for this Writer. This field
is required for record type SWR and optional for record type
OWR.
Writer Last Name 29 45 A C The last name of this writer. Note that if the submitter does not
have the ability to split first and last names, the entire name
should be entered in this field in the format “Last Name, First
Name” including the comma after the last name. This field is
required for record type SWR and optional for record type OWR.
Writer First Name 74 30 A O The first name of this writer along with all qualifying and middle
names.
Writer Unknown
Indicator
```
```
104 1 F C Indicates if the name of this writer is unknown. Note that this
field must be left blank for SWR records. For OWR records, this
field must be set to “Y” if the Writer Last Name is blank.
Writer
Designation Code
```
```
105 2 L C Code defining the role the writer played in the composition of
the work. These values reside in the Writer Designation Table.
This field is required for record type SWR and optional for record
type OWR.
Tax ID # 107 9 A O The number used to identify this writer for domestic tax
reporting.
Writer IPI
Name #
```
```
116 11 L O The IPI Name # assigned to this writer.
```
```
PR Affiliation
Society #
```
```
127 3 L O Number assigned to the Performing Rights Society with which
the writer is affiliated. These values reside on the Society Code
Table.
PR
Ownership Share
```
```
130 5 N
999v99
```
```
O Defines the percentage of the writer’s ownership of the
performance rights to the work. Within an individual SWR
record, this value can range from 0 to 100.0. Note that writers
both own and collect the performing right interest.
MR Society 135 3 L O Number assigned to the Mechanical Rights Society with which
the writer is affiliated. These values reside on the Society Code
Table.
MR
Ownership Share
```
```
138 5 N
999v99
```
```
O Defines the percentage of the writer’s ownership of the
mechanical rights to the work. Within an individual SPU record,
this value can range from 0 to 100.0.
```
```
Field Start Size Fmt Req Field Description
SR Society 143 3 L O Number assigned to the Mechanical Rights Society with which
the writer is affiliated. These values reside on the Society Code
Table.
SR
Ownership Share
```
```
146 5 N
999v99
```
```
O Defines the percentage of the writer’s ownership of the
synchronization rights to the work. Within an individual SPU
record, this value can range from 0 to 100.0.
```
 **_Society/Region Specific Fields_**

```
Reversionary
Indicator
```
```
151 1 F O Indicates writer involved in the claim.
Note that this flag only applies to societies that recognize
reversionary rights (for example, SOCAN).
First Recording
Refusal Ind
```
```
152 1 B O Indicates whether the submitter has refused to give authority
for the first recording. Note that this field is mandatory for
registrations with the UK societies.
Work For Hire
Indicator
```
```
153 1 B O Indicates whether or not this work was written for hire.
```
```
Filler 154 1 A O
```

 **_Version 2.0 Fields_**

```
Writer IPI Base
Number
```
```
155 13 L O The IPI base number assigned to this writer. These values reside
in the IPI database.
Personal Number 168 12 N O The personal number assigned to this writer in the country of
his/her residence.
```
 **_Version 2.1 Fields_**

```
USA License Ind. 180 1 L O Indicates that rights flow through SESAC/BMI/ASCAP/AMRA in
the US.
```
**Record Level Validation**

1. Edit has been removed.
2. Unless the total writers’ ownership shares is equal to 100% for each right (that is, the work is
   unpublished), each SWR record must be followed by at least one PWR record. (TR)
3. Edit has been removed.

**Field Level Validation**

1. If Record Type is equal to SWR, Interested Party # must be entered. (TR)
2. Submitters must ensure that the Interested Party # is unique within their system for both current and
   past controlled parties. (TR)
3. If Record Type is equal to SWR or Writer Unknown Indicator is not equal to “Y”, Writer Last Name must
   be entered. (TR)
4. If Record Type is equal to SWR, Writer Unknown Indicator must be blank. (TR)
5. If Record Type is equal to OWR, and Writer Unknown Indicator is entered, it must be equal to Y or N (FR
    - default to N)
6. If Record Type is equal to OWR, and Writer Unknown Indicator is equal to “Y”, Writer Last Name must be
   blank. (FR - default Writer Unknown Indicator to “N”)
7. For SWR records, Writer Designation Code must be entered. (TR)
8. If entered, Writer Designation Code must match an entry in the Writer Designation table. (TR)
9. If entered, Writer IPI Name # must match a writer entry in the IPI database. (FR – default to spaces)
10. If entered, PR Affiliation Society # must match an entry in the Society Code table. (FR - default to spaces)
11. PR Ownership Share must be numeric. The value must also be between 00000 (0%) and 10000 (100.00%).
    Note that the decimal point on this field is implied and should not be present within the field. (TR)
12. If entered, MR Affiliation Society # must match an entry in the Society Code table (FR - default to spaces)
13. MR Ownership Share must be numeric. The value must also be between 00000 (0%) and 10000
    (100.00%). Note that the decimal point on this field is implied and should not be present within the field
    (TR)
14. If entered, SR Affiliation Society # must match an entry in the Society Code table. (FR - default to spaces)
15. SR Ownership Share must be numeric. The value must also be between 00000 (0%) and 10000 (100.00%).
    Note that the decimal point on this field is implied and should not be present within the field. (TR)
16. If entered, Reversionary Indicator must be equal to Y, N, or U. (FR - default to spaces)


17. If entered, First Recording Refusal Ind must be equal to Y or N. (FR - default to spaces)
18. If entered, Work for Hire Indicator must be equal to ‘Y’ or ‘N’ (FR - default to spaces)
19. Edit has been removed.

_Version 1.1 Edits_

20. When Version equals “MOD”, if Writer Designation code equal “C” or “CA” or “A” and with zero shares,
    there must exist another SWR with non-zero shares and Writer Designation of “AR”, “TR”, “SA”, “SR” or
    “AD”. (TR)
21. If entered, Tax ID must be numeric. (FR - default to zeros)

_Version 2.0 Edits_

22. If Writer IPI Base Number is entered, it must match an entry in the IPI database. (FR)
23. Edit has been removed.

_Version 2.1 Edits_

24. If USA License Ind is entered, it must match a value in the USA License Indicator table. (FR)
25. Edit has been removed.
26. Edit has been removed.
27. For Registrations to SGAE only – The Writer Unknown Indicator must not be set to “Y”. (TR – SGAE only)
28. For Registrations to SGAE only – The Writer Last Name must not be set to “Unknown” or any other name
    indicating the Writer is not known (TR – SGAE only)
29. If entered, Writer Last Name must contain only valid ASCII characters from within the “Names” section
    of the allowed CIS character set. (TR)
30. If entered, Writer First Name must contain only valid ASCII characters from within the “Names” section
    of the allowed CIS character set. (TR)
31. For registrations to SACEM Societies only – If Record Type is OWR, the Writer Last Name must be entered,
    or else the Writer will be ignored. (RR – SACEM only).

### 5.11 NWN: Non-Roman Alphabet Writer Name

**Record Description**

This record identifies writer names in non-roman alphabets for this work. The language code is used to
identify the alphabet. This record can be used to identify the name of the writer in the preceding SWR/OWR
record.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = NWN
Interested Party # 20 9 A C Submitting publisher’s unique identifier for this Writer.
Writer Last Name 29 160 O M The last or single name of this writer.
Writer First Name 189 160 O O The first name of this writer.
Language Code 349 2 L O The Language code of the name – must be a valid code from the
Language Code Table.
```

**Record Level Validation**

1. This record must immediately follow an SWR that refers to the writer named here. (RR)

**Field Level Validation**

1. Interested Party # must be entered and must match the Interested Party # of the corresponding SWR
   record. (RR)
2. Writer Name must be entered. (RR)
3. Language code, if entered, must be a valid code from the Language Code Table. (FR)

### 5.12 SWT: Writer Territory of Control

### 5.13 OWT: Other Writer Collection

**Record Description**

This record was introduced in version 2.0. The SWT record specifies collection shares for a writer and the
application territory or territories for the collection shares. Note that SWT records must follow an SWR, NWN
(Non-Roman Alphabet Writer Name) or another SWT record and cannot be used with OWR records. One
SWT record must be used for every territory that is included or excluded. The most frequent case will be that
the writer collects one share percentage for the world (2136). It often happens that a writer collects a higher
percentage for his home territory only. In this case there would be an SPT with one percentage for the world;
an exclude SPT for the home territory with zero percentage; and an include SPT for the home territory with
the percentage collected there.

The Shares change flag alerts the recipient that the ownership will not equal collection for this writer. Usually
this is because the agreements permit shares to change when the work is sub-published. The sequence
number which was added in version 2.1 should run from 1 to the number of SWTs for each SWR.

The OWT record is used to record collection for non-controlled writers. OWT records are optional and should
not be treated as providing authoritative information. They can be useful in making clear the total collection
for a territory where shares change on sub-publication or where the territories of controlled collection vary
between chains.

An OWT can follow an OWR. It can never appear in a controlled chain (that is after a SWR).

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = SWT (writer Territory of Control) or OWT
(other writer collection).
Interested Party # 20 9 A C Submitting publisher’s unique identifier for this Writer.
PR Collection
Share
```
```
29 5 N
999v99
```
```
O Defines the percentage of the total royalty distributed for
performance of the work which will be collected by (paid to) the
writer within the above Territory. Within an SWT record, can be
a range from 0 to 100.00.
MR Collection
Share
```
```
34 5 N
999v99
```
```
O Defines the percentage of the total royalty distributed for sales
of CDs, Cassette Tapes, etc. in which the work is included which
will be collected by (paid to) the writer. Within an SWT record,
can be a range from 0 to 100.00.
```

```
SR Collection
Share
```
```
39 5 N
999v99
```
```
O Defines the percentage of the total royalty distributed for
Synchronization rights to the work which will be collected by
(paid to) the writer. Within an SWT record, can be a range from
0 to 100.00.
Inclusion/
Exclusion
Indicator
```
```
44 1 L M This is a marker which shows whether the territory specified in
this record is part of the territorial scope of the agreement or
not. Possible entries are I (= territory included) and E (= territory
excluded).
TIS Numeric Code 45 4 L M A territory within which this writer has the right to collect
payment for performance of this work. These values reside in
the TIS Code Table.
Shares change 49 1 B O If the shares for the writer interest change as a result of sub-
publication in this territory, set this field to “Y”.
```
 **_Version 2.1 Fields_**

```
Sequence # 50 3 N M A sequential number assigned to each territory following an
SWR or OWR
```
**Record Level Validation**

1. If the Inclusion/Exclusion Indicator is “I”, at least one of PR Collection Share, MR Collection Share, or SR
   Collection Share must be greater than zero. (TR)
2. Each Territory (TIS code) included on an SWT record can only be linked to one SWR for a given right. (TR)
3. If the Inclusion/Exclusion Indicator is “E”, all Collection Shares must be set to zero. (FR).
4. For Registrations to ASCAP only - The first SWT record immediately following an SWR must have the
   “Inclusion/Exclusion Indicator” set to “I”. (TR - ASCAP only)

**Field Level Validation**

1. An SWT record must follow an SWR, NWN or SWT record. (TR)
2. For an SWT record The Interested Party # must be entered and must be equal to the Interested Party #
   on the previous SWR record. (TR)
3. PR Collection Share must be between 00000 (0%) and 10000 (100%). (TR)
4. MR Collection Share must be between 00000 (0%) and 10000 (100%). (TR)
5. SR Collection Share must be between 00000 (0%) and 10000 (100%). (TR)
6. TIS Numeric Code must be entered and must match an entry in the TIS database. (TR)
7. Inclusion/Exclusion Indicator must be entered and must be either “E” for excluded or “I” for included.
   (TR)
8. If Shares change is entered, it must be set to “Y” or “N”. (FR - default – “N”)

_Version 2.1 Edits_

9. Sequence # must be present. (RR)
10. Sequence # must be 1 for the first SWT after an SWR and increment by 1 for each subsequent SWT. (RR)

_Version 2.2 Edits_

11. An OWT must follow an OWR, NWN, or OWT record. (TR)
12. An OWT must not follow a SWR in a controlled chain. (TR)


13. For an OWT record the Interested Party # must be equal to the Interested Party # on the previous OWR
    record. Note that this can be blank (TR)

### 5.14 PWR: Publisher For Writer

**Record Description**

The PWR record is used to indicate the publisher that represents the writer designated on the previous SWR
or OWR record for writers that are published (total writer ownership shares for each right are less than
100%). PWR is optional for OWR records. Use a separate PWR record to document each publisher that
represents the writer.

The writer to publisher agreement numbers are recorded in the PWR. The reason is that if two or more
writers for a work have an agreement with the same original publisher, it is possible to record each Society-
Assigned Agreement Number / Submitter Agreement Number in the PWR record that links that writer to the
original publisher. It is the society of the original publisher that assigns the society-assigned agreement
number to the writer to publisher agreement. The AGR contains a Society Agreement Number that is used
to link the agreement to a work registration.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = PWR (Publisher for Writer)
Publisher IP # 20 9 A C The publisher interested party number pointing back to the
SPU/OPU record for the original publisher/income participant
representing this writer.
Publisher Name 29 45 A C The name of the publisher indicated by the Interested Party #
field.
```
**_Version 2.0 Fields_**

```
Submitter
Agreement
Number
```
```
74 14 A O The unique number assigned to this agreement by the
submitter.
```
```
Society-Assigned
Agreement
Number
```
```
88 14 A O The unique number assigned to this agreement by the society
```
**_Version 2.1 Fields_**

```
Writer IP # 102 9 A C The writer interested party number pointing back to the
SWR/OWR record in an explicit link.
```
**_Version 2.2 Fields_**

```
Publisher
Sequence #
```
```
111 2 N M Reference to publisher chain this link relates to. This allows the
specific original publisher/income participant entry for this
relationship to be identified.
```
**Record Level Validation**

1. Edit has been removed

_Revised in Version 2.0_

2. Edit has been replaced
3. Edit has been replaced


**Field Level Validation**

1. Edit has been replaced.
   _2._ Edit has been replaced.

```
Version 2.0 Edits
```
3. If Submitter Agreement Number is entered, it must match the identifier for an original agreement on file
   with the society of the original publisher or it must match an identifier in the AGR transaction with
   agreement type of “OS” or “OG”. (FR - default to spaces)
4. If Society-Assigned Agreement Number is entered, it must match the identifier for an original agreement
   on file with the society of the original publisher. (FR - default to spaces)

_Version 2.1 Edits_

5. Edit has been replaced

_Version 2.2 Edits_

6. For registrations to MusicMark societies only - If SWR is BMI then any related publisher must be BMI or
   non-US PRO. If SWR is ASCAP then any related publisher must be ASCAP or non-US PRO
7. Publisher Sequence # must be entered and it must match the Publisher Sequence # of the relating
   Original SPU/OPU record. (TR)
8. For controlled chains (where the PWR follows an SWR) writer IP # must be entered.(TR)
9. For controlled chains (where the PWR follows an SWR) the Publisher IP # must be entered. (TR)
10. For controlled chains (where the PWR follows an SWR) the Publisher Name must be entered. (TR)
11. Writer IP # must match the Interested Party # entered on the preceding SWR/OWR record. Note this can
    be blank on an OWR record. (TR)
12. Publisher IP # must match the Interested Party # for the original publisher/income participant referenced
    by the publisher sequence # field. Note this can be blank on an OWR record. (TR)
13. Publisher Name must match the name of the original publisher/income participant referenced by the
    publisher sequence # field. (FR - default to the proper name referred to by the publisher sequence #).
    (FR)
14. For registrations to ICE Societies only – Agreement number missing on PWR (TR – ICE only)
15. For registrations to ICE Societies only – Agreement not found in ICE on quoted combination of Submitter,
    Assignor and Agreement number on PWR on transaction (TR – ICE only)
16. For registrations to ICE Societies only – Agreement is terminated (TR – ICE only)
17. For registrations to ICE Societies only – Agreement number on PWR does not exist in ICE (TR – ICE only)
18. For registrations to ICE Societies only – Claimed territory on transaction is not included on publisher's
    agreement in ICE (TR – ICE only)
19. For registrations to ICE Societies only – Original publisher on transaction does not own the quoted
    agreement number (TR – ICE only)


### 5. 15 ALT: Alternate Title................................................................................................................

**Record Description**

This record identifies alternate titles for this work. The language code is used to identify titles that have been
translated into a language other than the original. Note that this applies to translation of the title only - not
a translation of the work. Including record type VER would indicate a work translation.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = ALT (Alternate Title)
Alternate Title 20 60 A M AKA or pseudonym of the work title.
Title Type 80 2 L M Indicates the type of alternate title presented on this record.
These values reside in the Title Type Table.
Language Code 82 2 L C The code representing the language that this alternate title has
been translated into. These values reside in the Language Code
Table. A language Code Must be entered if the Title Type is equal
to “OL” or “AL”
```
**Field Level Validation**

1. Alternate Title must be entered. (RR)
2. Title Type must be entered and must match an entry in the Title Type table other than “OT” (Original
   Title). (FR - default to Alternative Title)
3. If Language Code is entered, it must match an entry in the Language Code Table. (RR)
4. The Alternate Title must contain only valid ASCII characters from within the “Titles” section of the allowed
   CIS character set unless the Title Type is equal to “OL” or “AL”. (RR)
5. If the Title Type is equal to “OL” or “AL”, the Alternate Title must contain only valid ASCII characters from
   within the “CWR National Titles” section of the allowed CIS character set. (RR).
6. If the Title Type is equal to “OL” or “AL”, Language Code must be entered. (RR)

### 5.16 NAT: Non-Roman Alphabet Title

**Record Description**

This record identifies titles in other alphabets for this work. The language code is used to identify the
alphabet. This record can be used to describe the original title of a work, and it can also be used to describe
alternate titles.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = NAT (Non-Roman Alphabet Title)
Title 20 640 A M The work title in non-Roman alphabet
Title Type 660 2 L M Indicates the type of title presented on this record (original,
alternate etc.). These values reside in the Title Type Table.
Language Code 662 2 L O The Language code of the title – must be a valid code from the
Language Code Table.
```

**Record Level Validation**

1. Only one instance of this record per work can contain the title type of original title.

**Field Level Validation**

1. Title must be entered. (RR)
2. Title Type must be entered and must match an entry in the Title Type table. (FR - default to Alternative
   Title)
3. If Language Code is entered, it must match an entry in the Language Code Table. (RR)

### 5.17 EWT: Entire Work Title for Excerpts

**Record Description**

If the work being registered is an excerpt, the EWT record is used to indicate the title of the complete work
from which the excerpt has been derived.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = EWT (Entire Work Title for Excerpts)
Entire Work Title 20 60 A M The title of the entire work from which this excerpt has been
derived.
ISWC of
Entire Work
```
```
80 11 A O The International Standard Work Code assigned to the work
from which this excerpt has been derived.
Language Code 91 2 L O The code defining the language in which the entire was originally
written. These values reside in the Language Code Table.
Writer 1 Last
Name
```
```
93 45 A O Last name of the original writer/composer of the work from
which this excerpt has been taken. Note that if the submitter
does not have the ability to split first and last names, the entire
name should be entered in this field in the format “Last Name,
First Name” including the comma after the last name.
Writer 1 First
Name
```
```
138 30 A O First name of the original writer/composer of the work from
which this excerpt has been taken.
Source 168 60 A O A description of the source from which the work was obtained.
```
 **_Version 2.0 Fields_**

```
Writer 1
IPI Name #
```
```
228 11 L O The IPI Name # assigned to the first writer of the entire work.
```
```
Writer 1
IPI Base Number
```
```
239 13 L O The IPI base number assigned to this writer. These values reside
in the IPI database.
Writer 2
Last Name
```
```
252 45 A O Last name of the second writer of this component. Note that if
the submitter does not have the ability to split first and last
names, the entire name should be entered in this field in the
format “Last Name, First Name” including the comma after the
last name.
Writer 2
First Name
```
```
297 30 A O First name of the second writer of this component.
```
```
Writer 2
IPI Name #
```
```
327 11 L O The IPI Name # assigned to the second writer of the entire work.
```
```
Writer 2
IPI Base Number
```
```
338 13 L O The IPI base number assigned to this writer. These values reside
in the IPI database.
Submitter
Work #
```
```
351 14 A O Number assigned to the work by the party submitting the file.
This number must be unique for the submitter.
```

**Record Level Validation**

1. Only one EWT record is allowed per transaction (TR).

**Field Level Validation**

1. Entire Work Title must be entered. (RR)
2. If entered, the ISWC of Entire Title must be a valid ISWC. (FR - default to spaces)
3. If entered, Language Code must match an entry in the Language Code table. (FR- default to spaces)

_Version 2.0 Edits_

4. If entered, Writer 1 IPI Name # must match an entry for the Writer in the IPI database. (FR - default to
   spaces)
5. If entered, Writer 2 IPI Name # must match an entry for the Writer in the IPI database. (FR - default to
   spaces)
6. If entered, Writer 1 IPI Base Number must match an entry in the IPI database. (FR- default to spaces)
7. If entered, Writer 2 IPI Base Number must match an entry in the IPI database. (FR- default to spaces)
8. If entered, the Submitter Work Number must uniquely identify the work. (FR- default to spaces)
9. Entire Work Title must contain only valid ASCII characters from within the ‘Titles’ section of the allowed
   CIS character set. (RR)
10. If entered, Writer 1 Last Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)
11. If entered, Writer 1 First Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)
12. If entered, Writer 2 Last Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)
13. If entered, Writer 2 First Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)

### 5.18 VER: Original Work Title for Versions

**Record Description**

If the work being registered is a version of another work, the VER record is used to indicate the title of the
original work from which the version has been derived.


**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = VER (Original Work Title for Versions)
Original Work
Title
```
```
20 60 A M Original title of the work from which this version was derived.
```
```
ISWC of Original
Work
```
```
80 11 A O The International Standard Work Code assigned to the work
from which this version has been derived.
Language Code 91 2 L O The code defining the language in which the work was originally
written. These values reside in the Language Code Table.
Writer 1 Last
Name
```
```
93 45 A O Last name of the original writer/composer of the work from
which this version has been derived. Note that if the submitter
does not have the ability to split first and last names, the entire
name should be entered in this field in the format “Last Name,
First Name” including the comma after the last name.
Writer 1 First
Name
```
```
138 30 A O First name of the original writer/composer of the work from
which this version has been derived.
Source 168 60 A O A description of the source from which the work was obtained.
```
**_Version 2.0 Fields_**

```
Writer 1 IPI Name
#
```
```
228 11 L O The IPI Name number assigned to the first writer of the original
work.
Writer 1 IPI Base
Number
```
```
239 13 L O The IPI base number assigned to this writer. These values reside
in the IPI database.
Writer 2 Last
Name
```
```
252 45 A O Last name of the second writer of the original work. Note that if
the submitter does not have the ability to split first and last
names, the entire name should be entered in this field in the
format “Last Name, First Name” including the comma after the
last name.
Writer 2 First
Name
```
```
297 30 A O First name of the second writer of the original work.
```
```
Writer 2 IPI Name
#
```
```
327 11 L O The IPI Name number assigned to the second writer of this
original work.
Writer 2 IPI Base
Number
```
```
338 13 L O The IPI base number assigned to this writer. These values reside
in the IPI database.
Submitter Work # 351 14 A O Number assigned to the original work by the party submitting
the file. This number must be unique for the submitter.
```
**Record Level Validation**

1. Only one VER record is allowed per transaction (TR).

**Field Level Validation**

1. Original Work Title must be entered. (RR)
2. If entered, the ISWC of Original Work must be a valid ISWC. (FR - default to spaces)
3. If entered, Language Code must match an entry in the Language Code table. (FR- default to spaces)

_Version 2.0 Edits_

4. If entered, Writer 1 IPI Name # must match an entry for the Writer in the IPI database. (FR - default to
   spaces)


5. If entered, Writer 2 IPI Name # must match an entry for the Writer in the IPI database. (FR - default to
   spaces)
6. If entered, Writer 1 IPI base Number must match an entry in the IPI database. (FR - default to spaces)
7. If entered, Writer 2 IPI base Number must match an entry in the IPI database. (FR - default to spaces)
8. If entered, the Submitter Work Number must uniquely identify the work. (FR - default to spaces)
9. Original Work Title must contain only valid ASCII characters from within the “Titles” section of the
   allowed CIS character set. (RR)
10. If entered, Writer 1 Last Name must contain only valid ASCII characters from within the “Names” section
    of the allowed CIS character set. (FR)
11. If entered, Writer 1 First Name must contain only valid ASCII characters from within the “Names” section
    of the allowed CIS character set. (FR)
12. If entered, Writer 2 Last Name must contain only valid ASCII characters from within the “Names” section
    of the allowed CIS character set. (FR)
13. If entered, Writer 2 First Name must contain only valid ASCII characters from within the “Names” section
    of the allowed CIS character set. (FR)

### 5.19 PER: Performing Artist

**Record Description**

The name of a person or group performing this work either in public or on a recording.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = PER (Performing Artist)
Performing Artist
Last Name
```
```
20 45 A M Last name of a person or full name of a group that has performed
the work on a recording or in public. Note that if the performer
is known by a single name, it should be entered in this field.
Performing Artist
First Name
```
```
65 30 A O First name associated with the performing artist identified in the
previous field.
Performing Artist
IPI
Name #
```
```
95 11 L O The IPI Name # corresponding to this performing artist. Values
reside in the IPI database.
```
**_Version 2.0 Field_**

```
Performing Artist
IPI
Base Number
```
```
106 13 L O The IPI base number assigned to this performing artist.
```
**Field Level Validation**

1. Performing Artist Last Name must be entered. (RR)
2. If Performing Artist IPI Name # is entered, it must match an entry on the IPI database. (FR - default to
   spaces)

_Version 2.0 Edit_


3. If Performing Artist IPI Base Number is entered, it must match an entry in the IPI database. (FR - default
   to spaces)
4. Performing Artist Last Name must contain only valid ASCII characters from within the ‘Names’ section of
   the allowed CIS character set (RR)
5. If entered, Performing Artist First Name must contain only valid ASCII characters from within the ‘Names’
   section of the allowed CIS character set (RR)

### 5.20 NPR: Performance Data in non-roman alphabet

**Record Description**

This record contains either the non-roman alphabet name of a person or group performing this work either
in public or on a recording, or the language/dialect of the performance. This is particularly important for
Chinese dialects such as Cantonese. Performance Dialect, if entered, must be a valid code from ISO 639-2(T).
(FR)

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = NPR
Performing Artist
Name
```
```
20 160 A C Name of a person or full name of a group that has performed
the work on a recording or in public. Note that if the performer
is known by a single name, it should be entered in this field.
Performing Artist
First Name
```
```
180 160 A O First name of a person that has performed the work on a
recording or in public..
Performing Artist
IPI Name #
```
```
340 11 L O The IPI Name # corresponding to this performing artist. Values
reside in the IPI database.
Performing Artist
IPI Base Number
```
```
351 13 L O The IPI base number assigned to this performing artist.
```
```
Language Code 364 2 L O The Language code of the name – must be a valid code from the
Language Code Table.
```
**_Version 2.1 Fields_**

```
Performance
Language
```
```
366 2 L C The language used in the performance – must be a valid code
from the Language Code Table
Performance
Dialect
```
```
368 3 L C The dialect used in the performance – must be a valid code from
ISO 639-2(T) e.g. if the performance is in Mandarin, YUE
Cantonese, MIN NAN or HAKKA, then use: CHN, YUH, CFR or HAK
```
**Field Level Validation**

1. Edit has been removed.
2. If Performing Artist IPI Name # is entered, it must match an entry on the IPI database. (FR - default to
   spaces)
3. If Performing Artist IPI Base Number is entered, it must match an entry in the IPI database. (FR - default
   to spaces)
4. Language code, if entered, must be a valid code from the Language Code Table. (FR)

_Version 2.1 Edits_

5. One of Performing Artist Name, Performance Language, or Performance Dialect must be entered. (RR)


6. Performance Language, if entered, must be a valid code from the Language Code Table. (FR)
7. Performance Dialect, if entered, must be a valid code from ISO 639-2(T). (FR)

### 5.21 REC: Recording Detail

**Record Description**

The REC record contains information on the first commercial release of the work.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = REC (Recording Detail)
Release Date 20 8 D O Date the work was or will be released for public
consumption. This date can be a past, present, or future
date.
Constant 28 60 A O Fill with blanks.
Release
Duration
```
```
88 6 T O Duration of the release of the work.
```
```
Constant 94 5 A O Fill with blanks.
```
 **_Version 2.0 Fields_**

```
Album Title 99 60 A O The name of the album in which the work was included if the
work was released as part of an album.
Album Label 159 60 A O Name of the organization that produced and released the album
in which the release of the work was included.
Release Catalog # 219 18 A O Number assigned by the organization releasing the album for
internal purposes such as sales and distribution tracking.
EAN 237 13 L O European Article Number of release (EAN-13)
ISRC 250 12 L O International Standard Recording Code of the recording of the
work on the release (according to ISO 3901).
Recording Format 262 1 L O Code that identifies the content of the recording: “A” (audio),
“V” (video).
Recording
Technique
```
```
263 1 L O Identifies the recording procedure: “A” (Analogue), “D” (Digital),
“U” (Unknown).
```
 **_Version 2.1 Field_**

```
Media Type 264 3 L O BIEM/CISAC code for media type.
```
 **_Version 2.2 Field_**

```
Recording Title 267 60 A O Title of the Sound Recording
Version Title 327 60 A O Title given to the version of the Sound Recording (for example:
“remixed by”)
Display Artist 387 60 A O Name of the artist of the Sound Recording
Record Label 447 60 A O Name of the organisation that produced the Sound Recording of
the work
ISRC Validity 507 20 L C If an ISRC is supplied, Indicates that the validity of the ISRC: “ Y ”
is valid, “ U ” the link is invalid, “ N ” the ISRC is invalid
Submitter
Recording
Identifier
```
```
527 14 A O The submitter’s unique identifier for this recording.
```

**Record Level Validation**

1. Edit has been removed.

**Field Level Validation**

1. At least one of the optional fields must be entered. (RR)
2. If entered, Release Date must be a valid date. (FR - default to zeros)
3. If entered, Release Duration must be a valid combination of hours, minutes, and seconds. (FR - default
   to zeros)
4. Edit has been removed.
5. Edit has been removed.
6. Edit has been removed.
7. Edit has been removed.
8. Edit has been removed.

_Version 2.0 Edits_

9. If entered, EAN must be a valid European Article Number of release. (FR- default to spaces)
10. If entered, ISRC must be a valid International Standard Recording Code. (FR- default to spaces)
11. If entered, Recording Format must be “A” for Audio or “V” for video. (FR - default to 'A')
12. If entered, Recording Technique must be “A” for analogue, “D” for digital or “U” for unknown. (FR -
    default to “U”)
13. For registration to SESAC only - If the work registration contains a SESAC interest, then Recording Format,
    Recording Technique, EP Cassette EP CD Flag, Album CD Album Cassette Flag, Single Flag, and Twelve
    Inch Single Flag must be entered. (TR – SESAC only)

_Version 2.1 Edits_

14. If entered, the Media type must match an entry from the BIEM/CISAC list of Media Types. (FR)
15. If entered, the First Album Title must contain only valid ASCII characters from within the ‘Titles’ section
    of the allowed CIS character set. (FR)

_Version 2.2 Edits_

16. If entered, Recording Title must contain only valid ASCII characters from within the ‘Titles’ section of the
    allowed CIS character set. (TR)
17. If entered, Version Title must contain only valid ASCII characters from within the ‘Titles’ section of the
    allowed CIS character set. (TR)
18. If an ISRC is supplied, ISRC Validity must be Y, N, or U. (RR)
19. If entered, the Submitter Recording Identifier must uniquely identify the recording.(RR)


### 5.22 ORN: Work Origin

**Record Description**

The purpose of this record is to describe the origin of the work. The origin may be a library, or an audio-visual
production or both. If the work originated in an AV production, additional information regarding the usage
of the work within the production can be helpful. Note that the cue sheet is always the final authority for
usage data. Many identifiers for the audio-visual production have been added with version 2.1 including the
reference as used in the CIS tool, AV Index.

Note a Library work that is only available via the Internet will still need to have the CD Identifier field filled
in. Any wording can be used in this field, such as “INTERNET”.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = ORN (Work Origin)
Intended Purpose 20 3 L M Indicates the type of production from which this work
originated. These values reside in the Intended Purpose Table.
Production Title 23 60 A C Name of the production from which this work originated. This
field is required when CWR Work Type on the NWR record
equals “FM”.
CD Identifier 83 15 A C If Intended Purpose is equal to LIB (Library Work), enter the
identifier associated with the CD upon which the work appears.
Cut Number 98 4 N O If Intended Purpose is equal to LIB (Library Work), enter the track
number on the CD Identifier where the work appears.
```
**_Version 2.1 Fields_**

```
Library 102 60 A C The library from which this work originated.
BLTVR 162 1 A O An indication of the primary use of the work within the AV
production. The definitive source for cue usage is the cue sheet.
```
```
Filler 163 25 N O Reserved for future use
```
```
Production # 188 12 A O The number generated by the production company to identify
the work.
Episode Title 200 60 A O Title of the episode from which this work originated.
Episode # 260 20 A O Number assigned to the episode by the producer.
Year of
Production
```
```
280 4 N O The year in which the production of the film or episode was
completed.
AVI Key... The following two fields form the unique key for the audio-visual
work within the AV Index.
AVI Society Code 284 3 N O The Society code of the society whose audio visual work detail
entry is referenced in the AV Index. These values reside on the
Society Code Table.
Audio-Visual
Number
```
```
287 15 A O Unique number used internally by the “owning” society to
identify the audio-visual work as referenced in the AV Index.
```
**_Version 2.2 Fields_**

```
V-ISAN Unique identifier for audio-visual production in which this work
is first used
ISAN 302 12 A O Root Segment
```

```
Episode 314 4 A O Episode or Part number
```
```
Check Digit 1 318 1 A O Check Character for the root and episode segment.
```
```
Version 319 8 A O Version Segment
Check Digit 2 327 1 A O Check Character for the Version Segment
EIDR 328 21 A O EIDR
EIDR 328 20 A O Root Number
Check Digit 348 1 A O Check Character
```
**Record Level Validation**

1. Edit has been removed.

**Field Level Validation**

1. Intended Purpose must be entered and must match an entry in the Intended Purpose Table. (RR)
2. Edit has been removed.
3. Edit has been removed.
4. If Intended Purpose is equal to “LIB” (Library Work), CD Identifier is required. (RR)

_Version 2.0 Edit_

5. If entered, Cut Number must be numeric. (FR)

_Version 2.1 Edit_

6. If entered, BLTVR must contain one of “B” for background, “L” for logo, or “T” for theme, “V” for visual
   or “R” for rolled up cues. (FR)
7. If entered, the V-ISAN must be valid. (FR)
8. If entered, Year of Production must be a valid year. (FR)
9. If entered, AVI Key must match an entry in the AV Index. (FR)
10. Production Title or Library must be entered. (RR)
11. If entered, Production Title must contain only valid ASCII characters from within the “Titles” section of
    the allowed CIS character set. (RR)
12. If entered, Episode Title must contain only valid ASCII characters from within the “Titles” section of the
    allowed CIS character set. (FR)
13. If Intended Purpose is equal to “FIL” or “TEL” then a Production Title must be entered (TR).

_Version 2.2 Edit_

14. If entered, the V-ISAN must be valid. (FR)
15. If Entered, The EIDR must be valid. (FR)


### 5.23 INS: Instrumentation Summary

**Record Description**

The INS record provides information on standard and non-standard instrumentation for serious works. If the
Musical Work Distribution Category is SER then instrumentation detail is required using one or more Standard
Instrumentation Type, one or more IND records, or one Instrumentation Description. The Instrumentation
Description is the least desirable, and should be used only if the other fields are not available. It is possible
to use both a Standard Instrumentation Type and one or more IND records to describe, for example, a wind
quintet and a piano. It is also possible to use both one or more Standard Instrumentation Type and one or
more IND records to describe, for example, a work written for two wind quintets and two pianos.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = INS (Instrumentation Summary)
Number of Voices 20 3 N O Indicates the number of independent parts included in this work.
Standard
Instrumentation
Type
```
```
23 3 L C Describes instrumentation if standard instrumentation is used
on this work. Note that this field is required if IND records are
not entered and if Instrumentation Description is blank. These
values reside in the Standard Instrumentation Table.
Instrumentation
Description
```
```
26 50 A C Describes instrumentation if non-standard instrumentation is
used on this work. Note that this field is required if IND records
are not entered and if Standard Instrumentation Type is blank.
```
**Record Level Validation**

1. Edit has been removed.
2. Edit has been removed.
3. Edit has been removed.
4. Edit has been removed.
5. Edit has been removed.

_Version 2.1 Edit_

6. Edit has been removed.
7. If Standard Instrumentation Type and/or IND records exist, the Instrumentation Description will be
   ignored. (FR)

### 5.24 IND: Instrumentation Detail

**Record Description**

 The IND record provides information on standard instruments or voices for serious works. If the Musical
Work Distribution Category is SER then instrumentation detail is required using one or more INS records as
well as IND records to describe the individual instruments (if any).

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = IND (Instrumentation Detail)
Instrument Code 20 3 L M Indicates the use of a specific instrument in this version of
instrumentation. These values reside in the Instrument Table.
```

```
Number of
Players
```
```
23 3 N O Indicates the number of players for the above instrument
```
**Record Level Validation**

1. If provided, IND records must follow an INS or IND record. (RR)

**Field Level Validation**

1. Instrument Code must be entered and must match an entry in the Instrument Table. (RR)
2. Number of Players must be numeric. (RR)

### 5.25 COM: Component

**Record Description**

If the work being registered is a composite work, the COM record will identify an individual component of
the composite.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = COM (Composite Component)
Title 20 60 A M The title of the original work from which a portion was taken and
included in the composite work.
ISWC of
Component
```
```
80 11 A O The International Standard Work Code assigned to the original
work from which a portion was taken and included in this
composite work.
Submitter
Work #
```
```
91 14 A O The number that the submitting party uses to refer to this
composite component.
Duration 105 6 T O The duration of this composite component.
Writer 1 Last
Name
```
```
111 45 A M Last name of the first writer of this component. Note that if the
submitter does not have the ability to split first and last names,
the entire name should be entered in this field in the format
“Last Name, First Name” including the comma after the last
name.
Writer 1 First
Name
```
```
156 30 A O First name of the first writer of this component.
```
```
Writer 1 IPI Name
#
```
```
186 11 L O The IPI Name number assigned to the first writer of this
component.
Writer 2 Last
Name
```
```
197 45 A O Last name of the second writer of this component. Note that if
the submitter does not have the ability to split first and last
names, the entire name should be entered in this field in the
format “Last Name, First Name” including the comma after the
last name.
Writer 2 First
Name
```
```
242 30 A O First name of the second writer of this component.
```
```
Writer 2 IPI Name
#
```
```
272 11 L O The IPI Name number assigned to the second writer of this
component.
```
 **_Version 2.0 Fields_**

```
Writer 1 IPI
Base Number
```
```
283 13 L O The IPI base number assigned to this writer. These values reside
in the IPI database.
Writer 2 IPI
Base Number
```
```
296 13 L O The IPI base number assigned to this writer. These values reside
in the IPI database.
```

**Record Level Validation**

1. COM records can only be entered for works where the Composite Type is entered. (RR)

**Field Level Validation**

1. Title must be entered. (RR)
2. If entered, the ISWC of Component must be a valid ISWC. (FR - default to blank)
3. If entered, Duration must consist of a valid combination of hours, minutes, and seconds. (FR - default to
   zeros)
4. Writer 1: Last Name must be entered. (RR)
5. If entered, Writer 1 IPI Name # must match an entry for the Writer in the IPI database. (FR - default to
   spaces)
6. If entered, Writer 2 IPI Name # must match an entry for the Writer in the IPI database. (FR - default to
   spaces)
7. Writer 2 Last Name is required if Writer 2 First Name is entered. (FR - default both names to blank)

_Version 2.0 Edits_

8. If entered, Writer 1 IPI base Number must match an entry in the IPI database. (FR- default to spaces)
9. If entered, Writer 2 IPI base Number must match an entry in the IPI database. (FR- default to spaces)
10. If entered, the Submitter Work Number must uniquely identify the work. (FR- default to spaces)
11. Title must contain only valid ASCII characters from within the ‘Titles’ section of the allowed CIS character
    set. (RR)
12. If entered, Writer 1 Last Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)
13. If entered, Writer 1 First Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)
14. If entered, Writer 2 Last Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)
15. If entered, Writer 2 First Name must contain only valid ASCII characters from within the ‘Names’ section
    of the allowed CIS character set. (FR)

### 5.26 MSG: Message

**Record Description**

MSG records are used to communicate the results of validation on individual transactions back to the
transaction’s originator. A table of messages used for CWR can be found in the CWR website. The table
contains all of the messages in this format. The message texts in the table have been reworded to make them
more easily understood, but the content is the same as in this manual. The combination of Record Type,
Message Level and Validation Number can be used to reference the error in this document. Message Type
provides you with the severity of the error. For example, if Message Type is equal to T, then the entire work
registration has been rejected.


**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = MSG (Message)
Message Type 20 1 L M Indicates whether this information is a warning, error, or for
information only. Values are F = Field Rejected, R = Record
Rejected, T = Transaction Rejected, G = Group Rejected, E =
Entire File Rejected
Original Record
Sequence #
```
```
21 8 N M The Record Sequence Number within the transaction associated
with this acknowledgment that caused the generation of this
message.
Record Type 29 3 A M The record type within the original transaction that caused
generation of this message.
Message Level 32 1 L M The level of editing that was responsible for generation of this
message. Values are E = Entire File, G = Group, T = Transaction,
R = Record, F = Field.
Validation
Number
```
```
33 3 A M Identifies the specific edit condition that generated this
message. Note that the combination of Record Type, Message
Level, and Validation Number points back to a condition within
this document.
Message Text 36 150 A M The text associated with this message.
```
**Field Level Validation**

1. Message Type must be entered and must be equal to “F”, “R”, “T”, “G” or “E”. (TR)
2. Original Record Sequence # must correspond to a value contained within the transaction that caused the
   generation of this message. (TR)
3. Message Level must be equal to “F”, “R”, “T”, “G” or “E”. (TR)
4. Message Text must be entered and must correspond to the validation referenced by Record Type,
   Message Level, and Validation Number within this document. (TR)

_Version 2.0 Edits_

5. Record Type must be entered, and must correspond to the record being validated. (TR)
6. Validation Number must be entered and must refer to a validation in this document. (TR)

_Note_ that the size of the Original Sequence Number has increased in version 2.0.

### 5.27 NET: Non-Roman Alphabet Entire Work Title for Excerpts

### 5.28 NCT: Non-Roman Alphabet Title for Components

### 5.29 NVT: Non-Roman Alphabet Original Title for Version

**Record Description**

This record identifies titles in other alphabets for this work. The language code is used to identify the
alphabet. This record can be used to describe the original title of a work, and it can also be used to describe
alternate titles.


**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = NET/NCT/NVT (Non-Roman Alphabet
Entire Work Title for Excerpts/ Non-Roman Alphabet Title
for Components/ Non-Roman Alphabet Original Title for
Versions)
Title 20 640 A M The title in non-Roman alphabet.
Language Code 660 2 L O The Language code of the title – must be a valid code from
the Language Code Table.
```
**Record Level Validation**

1. If Record Type is NET, this record must contain the title for the work described in the immediately
   preceding EWT. (RR)
2. If Record Type is NCT, this record must contain the title for the component described in the immediately
   preceding COM record. (RR)
3. If Record Type is NVT, this record must contain the title for the work described in the immediately
   preceding VER record. (RR)

**Field Level Validation**

1. Title must be entered. (RR)
2. If Language Code is entered, it must match an entry in the Language Code Table. (RR)

### 5.30 NOW: Non-Roman Alphabet Other Writer Name

**Record Description**

This record identifies writer names in non-roman alphabets for the work named in an EWT (entire work for
an excerpt), VER (original work for a version), or COM (component) record. The language code is used to
identify the alphabet.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = NOW
Writer Name 20 160 O M The last or single name of this writer.
Writer First Name 180 160 O M The first name of this writer.
Language Code 340 2 L O The Language code of the name – must be a valid code from the
Language Code Table.
Writer Position 342 1 L O The position of the writer in the corresponding EWT, VER, or
COM record
```
**Record Level Validation**

1. This record must follow an EWT, VER, COM, NET, NCT, NVT that refers to a work written by the writer
   named here. (RR)

**Field Level Validation**

1. Writer Name must be entered. (RR)
2. Language code, if entered, must be a valid code from the Language Code Table. (FR)


3. Writer Position, if entered, must be either “1” or “2”. (FR – default to “1”)

### 5.31 ARI: Additional Related Information

**Record Description**

This record may contain specific information or general information. The Work # is used to relate the work
being registered to an entry in an unidentified performance/use list, or to correct a work referenced in a cue
sheet, web site, etc. The free-text note contains general information addressed to one or all societies. It may
be used for important information concerning the work registration. Societies are not obliged to process ARI
records, even if the note is addressed to them. The note field should be used sparingly.

**Record Format**

```
Field Start Size Fmt Req Field Description
Record Prefix 1 19 A M Set Record Type = ARI (Additional Related Information)
Society # 20 3 L M Number assigned to the Society to which the Note is addressed.
These values reside Society Code Table. If the note is addressed
to all societies that use the ARI record, use “000”.
Work # 23 14 A C The Society work # that relates to this registration. It may have
been found on an unidentified list, or a website etc.
Type of Right 37 3 L M Indicates that this information relates to performing rights,
mechanical rights, sync. rights or all rights (ALL)
Subject Code 40 2 L C Subject of the ARI.
Note 42 160 A C Free text field pertaining to the type of right and subject
specified above.
```
**Record Level Validation**

1. Either Work # or Note must be entered. (RR)
   **Field Level Validation**
1. Society # must be entered and must match an entry in the Society Code table or “000”. (RR)
2. Type of right must be entered and must be a valid right or “ALL” for all. (RR)
3. Subject Code must be entered if Note is not blank, and must match an entry in the Subject Code
   table. (RR)

### 5.32 XRF: Work ID Cross Reference

**Record Description**

This Record contains identifiers issued by any organisation including but not limited to the intended recipient

of the file (though the principle work identifier should still be provided in NWR, ACK, REV and other headers).

```
Field Start Size Fmt Req Field Description
```
```
Record Prefix 1 19 A M Set Record Type = XRF (Work ID Cross Reference)
```
```
Organisation
Code
```
```
20 3 L M Number assigned to the Organisation (e.g. Society, publisher,
DSP etc...) which generated the Work Code. These values reside
in the Transmitter Code Table , or can be “ ISW ” for ISWC or “ ISR ”
for ISRC. Note: Do not use “000”or “099”.
```

```
Identifier 23 14 A M An identifier that relates to this work Transaction.
```
```
Identifier Type 37 1 L M The type of identifier ( “ W ” for Work, “ R ” for Recording, “ P ” for
Product, “V“ for Video)
```
```
Validity 38 1 F M Indicates whether the Identifier is valid or not: “ Y ” is valid, “ U ”
the link is invalid, “ N ” the identifier is invalid
```
**Field Level Validation**

1. Organisation Code must be entered and must match an entry in the Transmitter Code Table. (RR)
2. Identifier Type must be entered and must be equal to W, R, or P. (RR)
3. Validity Indicator must be entered and must be equal to Y, N or U. (RR)

## 6 CWR Data Structure

### 6.1 Definition of CWR using BNF

BNF is an acronym for "Backus Naur Form". John Backus and Peter Naur introduced for the first time a formal
notation to describe the syntax of a given language. Since then, almost every author of books on new
programming languages used it to specify the syntax rules of the language. It is also used to describe data
structures and interface construction. When applied rigorously, it provides an unambiguous definition of
what is valid (and by implication, what is not).

BNF is a way of defining structures algebraically. It uses a small number of symbols as defined in the following
table:

```
Symbol Meaning
< > Identifier: a name for something being defined or referenced. In practice, many now use
bold text to denote an identifier, normal text for text that is to be used literally
::= Is defined as
| Or
[] Optional statement
{} Repetitive Items
```
BNF is used below to describe the composition of a CWR file, with its headers and transactions, in a clear
unambiguous way.

**CWR_File** ::= HDR_Transmission_Header
{Group_Info}
TRL_Transmission_Trailer

**Group_Info** ::= GRH_Group_Header
{Transaction_Info}
GRT_Group_Trailer

**Transaction_Info** ::= AGR | NWR | REV | | ISW | ACK

**AGR_Transaction** ::= AGR_Transaction_Record
{Territory_Information}


**Territory_Information** ::= {TER_Territory}
{Assignor_Information}
{Acquirer_Information}

**Assignor_Information** ::= IPA_Interested_Party_of_Agreement
[NPA_Non-Roman_Alphabet_Agreement_Party_Name]

**Acquirer_Information** ::= IPA_Interested_Party_of_Agreement
[NPA_Non-Roman_Alphabet_Agreement_Party_Name]

**ACK_Transaction** ::= ACK_Transaction_Record
[{MSG_Records}] (precedes record pertaining to message)
AGR | NWR [EXC]| REV [EXC]
**NWR_Transaction** ::= NWR_Transaction_Record
[{Controlled_Publisher_Info}]
[{Other_Publisher_Information}]
[{Controlled_Writer_Information}]
[{Other_Writer_Information}]
[{ALT_Alternate_Title}]
[NAT_Non-Roman_Alphabet_Title]
[Information_for_Excerpts]
[Information_for_Versions] [{PER_Performing_Artist}]
[{NPR_Performing Artist_in_Non-Roman_Alphabet}]
[REC_Recording_Information]
[ORN_Work_Origin]
[{Instrumentation_Information}]
[{Information_for_Components }]
[{ARI_Additional_Related_Information}]
[{XRF_Work ID Cross Reference }]

**Controlled_Publisher_Information** ::= Original_Publisher_Information
[{Administrator_Information}]
[{Subpublisher_Information}]
[{SPU_Income_Participant_Publisher}]

**Original_Publisher_Information** ::= SPU_Original_Publisher | SPU_Income_Participant_Publisher
[NPN_Non-Roman_Alphabet_Publisher Name]
[{SPT_Territory_of_Control}]
[{OPT_Non_Controlled_Collection}]

**Administrator_Information** ::= SPU_Administrator
[NPN_Non-Roman_Alphabet_Publisher Name]
[{SPT_Territory_of_Control}]
[{OPT_Non_Controlled_Collection}]

**Subpublisher_Information** ::= SPU_Subpublisher
[NPN_Non-Roman_Alphabet_Publisher Name]
[{SPT_Territory_of_Control}]
[{OPT_Non_Controlled_Collection}]

**Other_Publisher_Information** ::= OPU_Other_Publisher
[NPN_Non-Roman_Alphabet_Publisher Name]
[{OPT_Non-Controlled_Collection}]

**Controlled_Writer_Information** ::= SWR_Controlled_Writer | SWR_Income_Participant_Writer
[NWN_Non-Roman_Alphabet_Writer Name]
[{SWT_Writer_Territory_of_Control}]
{PWR_Publisher_for_Writer}

**Other_Writer_Information** ::= OWR_Other_Writer | OWR_Other_Participant_Writer


```
[NWN_Non-Roman_Alphabet_Writer Name
[{OWT_Other_Writer_Territory_of_Control}]
{PWR_Publisher_for_Writer}
```
**Instrumentation_Information** ::= INS_Instrumentation_Summary
[{IND_Instrumentation_Detail}]

**Information_for_Excerpts** ::= EWT_Entire_Work_Title_for_Excerpts

```
[NET_Non-Roman_Alphabet_Title_for_Excerpts]
```
```
[{NOW_Non-Roman_Alphabet_Other_Writer_Name]]
```
**Information_for_Versions** ::= VER_Original_Work_Title_for_Versions
[NVT_Non-Roman_Alphabet_Original_Title_for_Versions]

```
[{NOW_Non-Roman_Alphabet_Other_Writer_Name]]
```
**Information_for_Components** ::= COM_Component

```
[NET_Non-Roman_Alphabet_Title_for_Components]
[{NOW_Non-Roman_Alphabet_Other_Writer_Name]]
```
**Note:** that the BNF definitions for REV, EXC, and ISW are the same as for NWR.

### 6.2 File skeleton sent from publisher to societies

This skeleton shows a combination of detail records that can be used in the various transactions.

HDR Transmission Header
GRH Group Header
AGR Agreement Transaction
TER Territory
IPA Assignor
IPA Acquirer
AGR Agreement Transaction
TER Territory
TER Territory
IPA Assignor
IPA Acquirer
IPA Acquirer
...
GRT Group Trailer
GRH Group Header
NWR New Work Registration Transaction
SPU Original Publisher
SPT Territory of Control
SPT Territory of Control
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection
SPU Administrator
SPT Territory of Control
SPT Territory of Control
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection
SPU Sub-publisher
SPT Territory of Control
SPT Territory of Control
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection


```
OPU Other Publisher
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection
SWR Writer
SWT Territory of Control
SWT Territory of Control
PWR Publisher for Writer
OWR Other Writer
OWT Other writer collection
OWT Other writer collection
PWR Publisher for Writer
ALT Alternate Title
EWT Entire Work Title for Excerpt
VER Original Work Title for Versions
PER Performing artist
PER Performing artist
REC Recording Detail
REC Recording Detail
ORN Work Origin
INS Instrumentation Summary
IND Instrumentation Detail
IND Instrumentation Detail
COM Component
COM Component
NWR New Work Registration Transaction
... Detail Records as described above
NWR New Work Registration Transaction
... Detail Records
GRT Group Trailer
GRH Group Header
REV Revised work registration Transaction & detail records- as for
NWR
REV Revised work registration Transaction & detail records
```
GRT Group Trailer
TRL Transmission Trailer

### 6.3 File skeleton sent from society to publisher

HDR Transmission Header
GRH Group Header
ACK Acknowledgement Transaction
MSG Message - precedes record of NWR/REV to which it refers
AGR Agreement Information including detail records sent by publisher

- may have supplemental data
  ACK Acknowledgement Transaction
  AGR Agreement Information
  ....
  GRT Group Trailer
  GRH Group Header
  ACK Acknowledgement Transaction
  MSG Message - precedes record of NWR/REV to which it refers
  NWR Work registration including detail records sent by publisher - may
  have supplemental data
  EXC Notification of work in conflict (if any)


```
ACK Acknowledgement Transaction
MSG Message
NWR Work registration
```
```
GRT Group Trailer
```
```
GRH Group Header
ACK Acknowledgement Transaction
MSG Message - precedes record of NWR/REV to which it refers
REV Work registration including detail records sent by publisher - may
have supplemental data
EXC Notification of work in conflict (if any)
ACK Acknowledgement Transaction
MSG Message
REV Work registration
```
```
GRT Group Trailer
GRH Group Header
ISW ISWC Notification Transaction & detail records - as for NWR
ISW ISWC Notification Transaction & detail records
```
GRT Group Trailer
TRL Transmission Trailer

**Note:** The groups are shown in one file in this skeleton, but it is possible to have only one group per file.

### 6.4 File skeleton containing Non-Roman Alphabet Records

This skeleton shows a sample of the records that can be used to register works containing non-Roman
alphabet data.

HDR Transmission Header
GRH Group Header
AGR Agreement Transaction
TER Territory
IPA Assignor
NPA Assignor Name in Non-Roman Alphabet
IPA Acquirer
AGR Agreement Transaction
TER Territory
TER Territory
IPA Assignor
IPA Acquirer
NPA Acquirer Name in Non-Roman Alphabet
IPA Acquirer
NPA Acquirer Name in Non-Roman Alphabet
...
GRT Group Trailer
GRH Group Header
NWR New Work Registration Transaction
SPU Original Publisher
NPN Original Publisher Name in Non-Roman Alphabet
SPT Territory of Control
SPT Territory of Control
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection


```
SPU Administrator
SPT Territory of Control
SPT Territory of Control
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection
SPU Sub-publisher
SPT Territory of Control
SPT Territory of Control
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection
OPU Other Publisher
OPT Publisher non-controlled collection
OPT Publisher non-controlled collection
SWR Writer
NWN Original Writer Name in Non-Roman Alphabet
SWT Territory of Control
SWR Territory of Control
PWR Publisher for Writer
PWR Publisher for Writer
OWR Other Writer
OWT Other writer collection
OWT Other writer collection
PWR Publisher for Writer
PWR Publisher for Writer
ALT Alternate Title
NAT Non-Roman Alphabet MainTitle
NAT Non-Roman Alphabet AlternateTitle
EWT Entire Work Title for Excerpt
NET Entire Work Title in Non-Roman Alphabet for Excerpt
NOW Other Writer Name in Non-Roman Alphabet for Excerpt
NOW Other Writer Name in Non-Roman Alphabet for Excerpt
NPR Performing artist in Non-Roman Alphabet
NPR Performing artist in Non-Roman Alphabet
INS Instrumentation Summary
IND Instrumentation Detail
IND Instrumentation Detail
NWR New Work Registration Transaction
```
GRT Group Trailer
TRL Transmission Trailer

## 7 Society-Specific Edits

The following section lists all of the society-specific edits that appear throughout this specification.

### 7.1 ABRAMUS and UBC

_NWR Transaction Level Validation Rules 40_

40. For registrations to ABRAMUS and UBC only – all SPU records for final Sub-Publisher’s collecting
    shares in the territory of Brazil must include the start date of the sub-publishing agreement in the
    Society-assigned Agreement Number field. The date should be left-justified in the format
    YYYYMMDD, and the rest of the field should be left blank. Invalid dates will be rejected. (TR –
    ABRAMUS and UBC only)


### 7.2 ASCAP

_NWR Transaction Level Validation Rules 28 & 29_

28. Edit Removed
29. Edit Removed

_SPT Record Level Validation Rule 6_

6. For registrations to ASCAP only – The first SPT record immediately following an SPU must have the
   ‘Inclusion/Exclusion Indicator’ set to ‘I’. (TR – ASCAP only)

_SWT Record Level Validation Rule 4_

4. For Registrations to ASCAP only - The first SWT record immediately following an SWR must have the
   'Inclusion/Exclusion Indicator' set to 'I'. (TR - ASCAP only)

_SPU Record Level Validation Rule 9, 13 &14_

9. For Registrations to ASCAP only – Where there is an ASCAP Original publisher there must be a
   collection territory that includes the United States within that chain of title. (TR – ASCAP only)
13. For registration ASCAP only – If the original publisher belongs to ASCAP then the sub-publisher or
    administrator in the chain for the US cannot belong to BMI or SESAC. (TR – ASCAP only)
14. For registration ASCAP only – If the original publisher belongs to BMI or SESAC then the sub-publisher
    or administrator in the chain for the US cannot belong to ASCAP. (TR – ASCAP only)

### 7.3 BMI

_NWR Field Level Validations Rule 21_

21. If Musical Work Distribution Category is equal to “JAZ” and BMI is represented on the work, duration
    must be greater than zero (TR – BMI only)

### 7.4 GEMA

_NWR Field Level Validation Rules 26, 27 & 28_

26. Edit Removed
27. Edit Removed
28. For registrations to GEMA only - If an ISWC number is entered and is part of the GEMA number block
    then that number must already be registered by GEMA. (FR –GEMA only – Replace with spaces).

_SPU Field Level Validation Rules 33 & 34_

33. For registrations to GEMA only - If Agreement Type is equal to “PS”, the Submitter Agreement
    Number must be entered and must match an entry in a corresponding AGR-record. (TR - GEMA only)
34. For registrations to GEMA only – If Record Type is equal to “SPU” and Publisher Type is equal to “SE”,
    an Agreement Type must be entered. (TR – GEMA only).

_NWR Transaction Level Validation Rules 30 & 31_

30. For registrations to GEMA only - A work transaction must contain at least one SPU with either “PR
    Affiliation society” or “MR Affiliation society” set to 035 (GEMA), otherwise the work will be rejected
    with the transaction status set to “NP” (TR – GEMA only).


31. For registrations to GEMA only – If all the Writer’s Last Names are entered as “DP” (i.e. Public
    Domain), then the work will be rejected with Transaction Status “NP”. (TR – GEMA only).

_NWR Field Level Validation Rules 26 & 27_

26. Edit Removed
27. Edit Removed

### 7.5 Harry Fox

_NWR Transaction Level Validation Rule 32_

32. For registrations to Harry Fox only – A transaction must contain at least one SWR record (TR – Harry
    Fox only).

### 7.6 ICE Societies

_NWR Transaction Level Validation Rule XX and XX_

1. For registrations to ICE Societies only – If writer designation code AR on work, a different writer's
   designation code must be C or CA (TR – ICE only)
2. For registrations to ICE Societies only – If writer designation code SA on work, a different writer's
   designation code must be A or CA (TR – ICE only)

_SPU Transaction Level Validation Rule XX, XX, XX, XX and XX_

1. For registrations to ICE Societies only – Agreement number missing on SPU (TR – ICE only)
2. For registrations to ICE Societies only – Agreement not found in ICE on quoted combination of
   Submitter, Assignor and Agreement number on SPU on transaction (TR – ICE only)
3. For registrations to ICE Societies only – Agreement is terminated (TR – ICE only)
4. For registrations to ICE Societies only – Agreement number on SPU does not exist in ICE (TR – ICE
   only)
5. For registrations to ICE Societies only – Claimed territory on transaction is not included on
   publisher's agreement in ICE (TR – ICE only)

_PWR Transaction Level Validation Rule XX, XX, XX, XX, XX and XX_

1. For registrations to ICE Societies only – Agreement number missing on PWR (TR – ICE only)
2. For registrations to ICE Societies only – Agreement not found in ICE on quoted combination of
   Submitter, Assignor and Agreement number on PWR on transaction (TR – ICE only)
3. For registrations to ICE Societies only – Agreement is terminated (TR – ICE only)
4. For registrations to ICE Societies only – Agreement number on PWR does not exist in ICE (TR – ICE
   only)
5. For registrations to ICE Societies only – Claimed territory on transaction is not included on
   publisher's agreement in ICE (TR – ICE only)
6. For registrations to ICE Societies only – Original publisher on transaction does not own the quoted
   agreement number (TR – ICE only)


### 7.7 MusicMark

_NWR Field Level Validations Rule nn_

22. If SWR is BMI then any related publisher must be BMI or non-US PRO. If SWR is ASCAP then any related
    publisher must be ASCAP or non-US PRO (TR- MusicMark only)

### 7.8 SACEM

_NWR Transaction Level Validation Rule 41_

41. For registrations to SACEM only - A work transaction must contain at least one SPU with either “PR
    Affiliation society” or “MR Affiliation society” set to 058 (SACEM), otherwise the work will be rejected
    with the transaction status set to “NP”. (TR – SACEM only).

_NWR Field Level Validation Rule 29_

29. For registrations to SACEM only - If an ISWC number is entered and is part of the SACEM number
    block then that number must already be registered by SACEM. (FR – SACEM only – Replace with
    spaces).

_SPU Field Level Validation Rules 38 & 39_

38. For registrations to SACEM only – If Record Type is equal to “SPU” and Publisher Type is equal to
    “SE”, an Agreement Type must be entered. All SPU records for final Sub-Publishers in the FR must
    supply pre-notified Society-assigned Agreement Number. (TR – SACEM only).
39. For registrations to SACEM Societies only – If the publisher is a SACEM member and the Society-
    Assigned Agreement Number is entered, it must match an agreement number on file with the SACEM
    Society. (TR – SACEM only).

```
Note for rule 39 : The first 7 characters must be used for the Society Agreement Number linking the
Original Publisher to the Administrator. The last 7 characters must be used for the Society Agreement
Number linking the administrator to the Sub Publisher. (PG)
```
46. For registrations to SACEM only: The IPI-name-number provided for the Original Publisher SPU must
    be equal to the assignor IPI-name-number for the agreement represented by the first 7 characters of
    the Society Provided Agreement Number.
47. For registrations to SACEM only: The IPI-name-number provided for the Sub-Publisher SPU must be
    equal to the assignee IPI-name-number for the agreement represented by the last 7 characters of the
    Society Provided Agreement Number.

_SWR/OWR Field Level Validation Rule 31_

31. For registrations to SACEM Societies only – If Record Type is OWR, the Writer Last Name must be
    entered, or else the Writer will be ignored (RR – SACEM only).

### 7.9 SESAC

_REC Transaction Level Validation Rule 13_

13. If the work registration contains a SESAC interest then Recording Format, Recording Technique, EP
    Cassette EP CD Flag, Album CD Album Cassette Flag, Single Flag, and Twelve Inch Single Flag must be
    entered. (TR – SESAC only)


### 7.10 SGAE

_NWR Transaction Level Validation Rule 35, 36, 37 & 38_

35. For registrations to SGAE only - All SPU records for Spanish sub-publishers must include a Society-
    assigned Agreement Number, or a Specific agreement must already have been notified to SGAE (TR
    - SGAE only)
36. For Registrations to SGAE only - For SGAE territories, the total Ownership shares of non-controlled
    right-owners (OPUs and OWRs) and the Collection shares of controlled right-owners (SPTs and SWTs)
    must equal 100% for each right. Note that a tolerance of plus or minus 00006 (0.06%) is allowed (TR
    - SGAE only)
37. For Registrations to SGAE only - For SGAE territories, the total of PR Ownership shares of Non-
    Controlled publishers (OPUs) and the PR Collection shares of controlled publishers (SPTs) must be
    less or equal to 50%. Note that a tolerance of plus or minus 00006 (0.06%) is allowed (TR – SGAE
    only)
38. NWR Transaction Level Validation Rule: For registrations to SGAE only – If all the Writers are Public
    Domain, then the work will be rejected (TR – SGAE only)
39. Part of the agreements hierarchy missed. There is no agreements link registered (notified) between
    original publisher and the submitter of the work (TR – SGAE only).
40. The sub-publisher of the work is not the registered (notified) for the original publisher (TR – SGAE
    only).
41. The shares for the sub-publisher are different from the shares registered (notified) in the agreement
    (TR – SGAE only).

_SPU Record Level Validation Rules 11 & 12_

11. For registrations to SGAE only - SPU records for final Sub-Publishers with SGAE interest must include
    a pre-notified Society-assigned Agreement number unless the Agreement Type field is set to “OS” or
    “PS”. (TR – SGAE only)
12. For registrations to SGAE only - If the record is of SGAE interest then it must include an Agreement
    Type. (TR – SGAE only)

_SWR Field Level Validation Rules 27 & 28_

27. For Registrations to SGAE only – The Writer Unknown Indicator must not be set to “Y”. (TR – SGAE
    only)
28. For Registrations to SGAE only – The Writer Last Name must not be set to “Unknown” or any other
    name indicating the Writer is not known. (TR – SGAE only)

### 7.11 SIAE

NWR Transaction Level Validation Rule 42

3. For registration SIAE only – There must be at least one writer designation code of C or CA on a
   transaction. (TR – SIAE only)


## 8 Previous Revisions

```
Number of
revision
```
```
Date Main modifications
```
2.2R2 09/2019 (^) ➢ Update to transmission header record to account for IPI Name Numbers
(IPNN) greater than 9 digits.
➢ OPU validation updates.
➢ GRH record version number update.
➢ MSG record example removal.
➢ Addition of missing XREF record in Transaction Format table and file
skeletons.
➢ Update to XRF field description for Organisation Code.
2.2R1 04/2017 (^) ➢ Correction of missing edit PWRF006 and renumbering the edits below.
2.2 05/2015
04/20 16
➢ HDR: new fields for, version, revision, software package and package
release number
➢ SPU: IPI Name Numbers made mandatory for a submitter’s own IPI
Name Numbers
➢ PWR removed edit to allow PWR to follow OWR
➢ PWR to SPU Sequence Number link
➢ OPT New ‘non-controlled publisher collection’ record (optional)
➢ OWT new ‘other writer collection’ record (optional)
➢ REC New optional fields for Sound Recording Title, Sound Recording
version title, Record Label, Display Artist
➢ ORN removed previously invalid ISAN field and note relating to its use
➢ ORN New optional Field for ISAN and EIDR
➢ XRF new Optional Record
➢ Society specific edits agreed at the New York meeting added
➢ Information Services Committee (Lisbon, 14/04/2016) approves Version
2.2


