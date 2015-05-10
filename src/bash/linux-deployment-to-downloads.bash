#!/bin/bash
# A specialized script which copies the contents of deployment tar files to a given location, suitable
# to be served as 
tar_file=${1:?Must be .tar file produced by any of the *-deploy.sh scripts}
program_type=${2:?Is the type of the program contained in the tar file, e.g. cli}
version=${3:?The program version contained in the tar file, e.g. 0.2.0 (no leading v)}
os_name=${4:?The OS name on which the contents of the tar files was produced, e.g. osx, ubuntu}
base_dir=${5:?Is the root path of the download directory, e.g. /var/www/downloads}


dest_dir=${base_dir}/google.rs/${program_type}/${version}/${os_name}
mkdir -p ${dest_dir} || exit $?
cd ${dest_dir} && tar -xzvf ${tar_file} || exit $?
for file in `find . -executable -type f`; do 
	program_tar_file=${file}.tar.gz
	tar -czf ${program_tar_file} $file && rm $file && echo "Created ${program_tar_file}"
done

echo Extracted programs from $tar_file to ${dest_dir}
