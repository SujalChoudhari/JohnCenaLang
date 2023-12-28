# John Cena Lang

John Cena Lang is a programming language I created, drawing inspiration from the fascinating concept of Turing machines. In this language, instructions are triggered based on the number of spaces in the code, offering a unique and Turing machine-inspired approach to programming.

## Getting Started

To run a John Cena Lang program, follow these steps:

1. **Prerequisites:** Ensure you have Rust installed on your system.

2. **Clone the Repository:**
   ```bash
   git clone https://github.com/SujalChoudhari/JohnCenaLang.git
   cd JohnCenaLang
   ```

3. **Run a Program:**
   ```bash
   cargo run path/to/your/program.jc
   ```

   Replace `path/to/your/program.jc` with the path to your John Cena Lang program file.

## Language Features

John Cena Lang supports the following instructions:

- 1: `LEFT`: Move the needle in the environment to the left.
- 2: `RIGHT`: Move the needle in the environment to the right.
- 3: `INCR`: Increment the value at the current memory location.
- 4: `DECR`: Decrement the value at the current memory location.
- 5: `READ`: Get and print the current memory value.
- 6: `READCHAR`: Get and print the current memory value as a character.
- 7: `MUL10`: Set a multiplier for the next increment or decrement operation to 10.
- 8: `MUL100`: Set a multiplier for the next increment or decrement operation to 100.
- 9: `WRITE`: Set the current memory value based on user input.

Each number corresponds to number of spaces between lines.
A line can ended by using `\n` or `\t`. As one line ends other automatically starts.

## Turing Machine Inspiration

John Cena Lang's behavior is inspired by the fundamental principles of Turing machines. By using whitespace as a means to trigger instructions, it reflects the simplicity and universality that characterize Turing machines, providing a novel and intriguing way to approach programming.

## Example Program

Here is a simple example program in John Cena Lang:

```jc
        	   	       	    	       	    	       	    	   	   	      
 	        	   	       	    	       	    	       	    	    	      
 	        	   	       	   	    	    	      
 	        	   	       	   	    	    	      
 	        	   	       	   	   	      
 	       	   	       	   	       	   	   	   	      
 	        	   	       	   	       	   	    	      
 	        	   	       	   	   	      
 	        	   	       	   	   	   	   	   	      
 	        	   	       	   	    	    	      
 	        	   	      
 	       	   	       	   	       	   	   	   	   	      
```

This program showcases the language's capability to increment values, apply multipliers, interact with user input, and navigate through the memory space.

Feel free to explore and experiment with the language to create your own unique programs!

## Contributing

If you're interested in contributing to John Cena Lang, fork the repository and submit a pull request. I welcome any improvements, bug fixes, or new features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.