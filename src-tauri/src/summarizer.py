#file to summarize text files using a pre-trained model
# This script uses the Hugging Face Transformers library to summarize text files.
# It loads a pre-trained model and uses it to generate summaries of the input text.

# takes in a text file, summarizes it, and saves the summary to another text file

#need to figure out dependencies to work with rest of project
import transformers
from transformers import pipeline
import os
import json
import re
import logging
from typing import List, Dict

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    logger = logging.getLogger(__name__)

    # Load the summarizer model
    summarizer = pipeline("summarization", model="facebook/bart-large-cnn")

    #we only want to take in a text file and summarize it
    def summarize_text(text: str, max_length: int = 130, min_length: int = 10) -> str:
        """
        Summarize the input text using the summarizer model.
        """
        try:
            summary = summarizer(text, max_length=max_length, min_length=min_length, do_sample=False)
            return summary[0]['summary_text']
        except Exception as e:
            logger.error(f"Error summarizing text: {e}")
            return text  # Return the original text if summarization fails
    #read text from a file
    def read_text_file(file_path: str) -> str:
        """
        Read text from a file.
        """
        with open(file_path, 'r', encoding='utf-8') as file:
            return file.read()
    #write text to a file
    def write_text_file(file_path: str, text: str):
        """
        Write text to a file.
        """
        with open(file_path, 'w', encoding='utf-8') as file:
            file.write(text)
    #summarize a file
    def summarize_file(input_file: str, output_file: str):
        """
        Summarize a file and save the summary to another file.
        """
        text = read_text_file(input_file)
        summary = summarize_text(text)
        write_text_file(output_file, summary)
        logger.info(f"Summarized {input_file} and saved to {output_file}")

    #main function to handle command line arguments
    def main():
        import argparse
        parser = argparse.ArgumentParser(description="Summarize text files.")
        parser.add_argument('input_file', type=str, help="Path to the input text file.")
        parser.add_argument('output_file', type=str, help="Path to save the summarized text.")
        args = parser.parse_args()

        summarize_file(args.input_file, args.output_file)
    #main function if the script is executed directly
    if __name__ == "__main__":
        main()
