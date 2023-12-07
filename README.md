# CSV-Table
A rust program that reads a csv file into a memory table. will be used in my csv database<br>
currently allows you to read a csv table into a table struct.<br>
a csv file should be formatted in a specific way:<br>
the first line is the metadata line. this contains info like column name and column type.<br>
each column should be separated by a comma and the column data should be formatted like:<br>
ColumnName:Type<br>
each row after this is just the actual table data<br>



TODO LIST:<br>
-Create a Database struct(stores multiple tables at once. csv files should be stored in one "DataBase" directory)
-allow for a setup file
-create access methods and editing methods(eg SELECT,INSERT,DELETE,UPDATE)
-combine with a custom SQL parser
-allow for conditional access/editing methods
-use supporting HashMaps and binary trees to speed up access times when using conditional statements