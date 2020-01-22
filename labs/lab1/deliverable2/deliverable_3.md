1) it generates a candidate integer from a range 0 to n, where n is a parameter
   of the function. It then sets the first bit of the candidate number to 1 so
   that we only check odd numbers, since no even numbers are prime. (well,
   actually 2 is a prime number but I it does not get considered here)

   if the candidate is a prime number, the candidate number is returned
