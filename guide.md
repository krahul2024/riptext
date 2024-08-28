1. Search
    * Single file operations, search for a pattern on different flags(case matching, whole/partial). 
    * Recusrive
        - for file names, 
        - for matching patterns in text in all files 
        - 
### **Single File Processing:**

- Search
- Pattern Matching
- Replacement
- Insertion
- Deletion
- Transformation
- Sorting
- Extraction
- Filtering
- Counting
- Text Manipulation
- Highlighting

### **Multiple File Processing:**
- Recursive Search
- Batch Replacement
- Batch Insertion
- Batch Deletion
- File Listing
- Filtering by File Type
- Filtering by Size
- Filtering by Date
- File Renaming
- File Moving
- File Copying
- Depth Handling
- Compression
- Archiving
- Permission Management
- Symlink Handling

### **Additional Functionalities:**
- Metadata Retrieval
- Custom Filters
- Comparison
- Logging
- Interactive Mode
- Preview Changes
- Custom Rules
- Synchronization
- Backup Creation
- File Splitting
- File Merging
- Directory Creation/Deletion
- Disk Usage Reporting


### **1. `find`**

- **File Searching:**
  - Search for files and directories in a directory hierarchy.
  - Search by name, type, size, modification date, and other attributes.
  
- **File Operations:**
  - Execute commands on found files (e.g., delete, move, or copy).
  - Print file paths or other attributes.

- **Depth Control:**
  - Control the depth of directory traversal.

### **2. `grep`**

- **Pattern Matching:**
  - Search for lines matching a specified pattern within files.
  - Support for regular expressions (regex) for complex patterns.

- **Inversion:**
  - Display lines that do not match the pattern.

- **Context Control:**
  - Show lines before, after, or surrounding matching lines.

- **Counting:**
  - Count the number of matching lines.

- **File Searching:**
  - Search within multiple files and directories.

### **3. `awk`**

- **Text Processing:**
  - Process and analyze text line-by-line and field-by-field.
  
- **Field Extraction:**
  - Extract specific fields or columns from text based on delimiters.

- **Text Manipulation:**
  - Perform calculations, formatting, and transformations on text data.

- **Pattern Matching:**
  - Search and apply actions based on patterns in the text.

- **Reporting:**
  - Generate formatted reports and summaries from text data.

### **4. `sed`**

- **Stream Editing:**
  - Perform basic text transformations on an input stream (files or stdin).
  
- **Substitution:**
  - Replace occurrences of a pattern with new text.

- **Deletion:**
  - Delete lines matching a specific pattern.

- **Insertion:**
  - Insert new text before or after lines matching a pattern.

- **Transformation:**
  - Apply text transformations, such as changing text case or modifying characters.

- **Printing:**
  - Print specific lines or ranges of lines from the input.
