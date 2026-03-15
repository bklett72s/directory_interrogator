# directory_interrogator  

## Purpose  
  &nbsp;Purpose of this project is to develop a memory safe method of reviewing directories to gain a Bill of Materials (BOM)  
   and present it to the user for detailed knowledge of a directoires contents.  
## Use
  &nbsp;For now, the application supports the use of Command Line Interface (CLI) arguments and a very lightweight  
  Graphic User Interface (GUI). The interface is just the application calling the systems file explorer.  
  Once a directory for scanning and output are designated. The application will walk the directory looking at the  
  files along with their size, type, extension, and generate a hash value for each file. If an archive is  
  discovered, the application will proceed to unpack it in a temporary scratch space and continue the process.  

## CLI Arguments
  &nbsp;-directory, -d: Directory to be scanned  
  &nbsp;-output, -o: Directory to write BOM.
