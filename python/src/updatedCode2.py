"""A multiprocessing benchmark that runs inside of AWS Lambda.

Calculate the sum of N numbers and print the result in loop.
"""
import time

def sum_numbers(count, loop):
    """Sum numbers."""
    numbers = list(range(count))
    #print the sum and the id of the process
    print(f"Sum: {sum(numbers)} | loop ID: {loop}")

#write an aws lambda handler function
def lambda_handler(event, context):
    """Lambda handler function."""
    # Measure the time it takes to spawn the processes.
    start = time.time()
    loops = event["loops"]
    count = event["count"]

    # Create 25 processes.

    for i in range(loops):
        sum_numbers(count, i)
    # Start the processes.
    
    end = time.time()
    # Print the results.
    print(f"Time to spawn processes: {end - start}")
    print(f"Time to complete processes: {end - start}")