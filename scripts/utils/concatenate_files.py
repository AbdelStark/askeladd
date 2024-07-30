import os
import shutil
import argparse

def concatenate_files(root_dir, extensions, output_file):
    """
    Concatenates all files with given extensions in a directory and its subdirectories into a single file.
    Includes the full path of each file before appending its content, prefixed with '//' as a comment.

    Args:
    root_dir (str): The root directory to search for files.
    extensions (list): A list of file extensions to look for.
    output_file (str): The path to the output file where the content will be concatenated.
    """
    with open(output_file, 'wb') as outfile:
        # Walk through all directories and files in the root directory
        for dirpath, dirnames, filenames in os.walk(root_dir):
            for filename in filenames:
                # Check if the file ends with any of the given extensions
                if any(filename.endswith(ext) for ext in extensions):
                    file_path = os.path.join(dirpath, filename)
                    # Write the full file path as a comment before the content
                    outfile.write(f"// {file_path}\n".encode())
                    # Open each file in binary mode and append its content to the output file
                    with open(file_path, 'rb') as infile:
                        shutil.copyfileobj(infile, outfile)
                        # Ensure there is a newline after each file's content (optional, for readability)
                        outfile.write(b'\n')
                    print(f"Appended {file_path} to {output_file}")

def main():
    parser = argparse.ArgumentParser(description='Concatenate files with specific extensions from a directory into a single file.')
    parser.add_argument('root_dir', type=str, help='The root directory to search for files')
    parser.add_argument('extensions', type=str, help='A comma-separated list of file extensions to look for')
    parser.add_argument('output_file', type=str, help='The file where the content will be concatenated')
    
    args = parser.parse_args()
    extensions = args.extensions.split(',')

    concatenate_files(args.root_dir, extensions, args.output_file)

if __name__ == '__main__':
    main()
