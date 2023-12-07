# CSV-Table
A rust program that reads a csv file into a memory table. will be used in my csv database<br>
currently allows you to read a csv table into a table struct.<br>
a csv file should be formated in a specific way:<br>
the first line is the metadata line. this contains info like column name and column type.<br>
each column should be seperated by a comma and the column data should be formated like:<br>
ColumnName:Type<br>
each row after this is just the actual table data
