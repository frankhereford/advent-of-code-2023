#!/bin/bash

# Data file to store package temperature
datafile="/tmp/cpu_package_temp_data.txt"

# Clear previous data file
echo "" > $datafile

while true; do
  # Get package temperature from sensors output and append to data file
  packageTemp=$(sensors | grep 'Package id 0:' | awk '{print $4}' | cut -d '+' -f 2 | cut -d '.' -f 1)
  echo $packageTemp >> $datafile
  
  # Plot using gnuplot
  gnuplot -p -e "set terminal dumb size $(tput cols),$(tput lines); plot '< tail -n 120 $datafile' with lines"



  # Sleep for 1 second
  sleep 1
  
  # Clear terminal for next plot
  clear
done

