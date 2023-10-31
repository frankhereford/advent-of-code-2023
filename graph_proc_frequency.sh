#!/bin/bash

# Data file to store average MHz
datafile="/tmp/cpu_mhz_data.txt"

# Clear previous data file
echo "" > $datafile

while true; do
  # Get average MHz and append to data file
  awkMHz=$(cat /proc/cpuinfo | grep 'MHz' | awk '{sum+=$4; count++} END {print sum/count}')
  echo $awkMHz >> $datafile
  
  # Plot using gnuplot
  gnuplot -p -e "set terminal dumb size $(tput cols),$(tput lines); plot '< tail -n 120 $datafile' with lines"



  #gnuplot -p -e "set terminal dumb; plot '$datafile' with lines"
  
  # Sleep for 1 second
  sleep 1
  
  # Clear terminal for next plot
  clear
done

