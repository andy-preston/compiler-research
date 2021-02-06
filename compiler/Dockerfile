FROM ubuntu:bionic AS buildComp

ENV TERM=linux
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /usr/local/src

##################################
#                                #
# Install the build dependencies #
#                                #
##################################

RUN apt-get update && \
    apt-get -y --no-install-recommends install \
        binutils \
        binutils-arm-none-eabi \
        ca-certificates \
        gcc \
        make \
        patch \
        subversion \
        tar \
        wget && \
    apt-get -y --purge remove cmdtest && \
    apt-get -y --purge autoremove && \
    apt-get -y upgrade && \
    apt-get clean && \
    rm -rf \
        /var/lib/apt/lists/* \
        /tmp/* \
        /var/tmp/* \
        /usr/src/doc/*

##################################################################
#                                                                #
# Get an x86 binary compiler to bootstrap the arm cross-compiler #
#                                                                #
##################################################################

RUN mkdir -p fpc && \
    cd fpc && \
    wget https://sourceforge.net/projects/freepascal/files/Linux/3.2.0/fpc-3.2.0-x86_64-linux.tar && \
    tar xvf fpc-3.2.0-x86_64-linux.tar && \
    cd fpc-3.2.0-x86_64-linux && \
    rm demo.tar.gz doc-pdf.tar.gz && \
    echo /usr/local | ./install.sh && \
    cd && \
    rm -rf /usr/local/src/* && \
    fpc -iV

#######################################
#                                     #
# Build the actual ARM cross compiler #
#                                     #
#######################################

COPY free_pascal.patch /usr/local/src

RUN svn co http://svn.freepascal.org/svn/fpc/trunk fpc && \
    cd fpc && \
    rm -rf .svn && \
    patch -p0 -i ../free_pascal.patch && \
    tar cvf /usr/local/share/rtl.tar rtl/embedded/arm/* && \
    ln -s /usr/bin/arm-none-eabi-as /usr/bin/arm-embedded-as && \
    make buildbase installbase CROSSINSTALL=1 OS_TARGET=embedded \
        CPU_TARGET=arm SUBARCH=armv7em && \
    rm -rf /usr/local/src/* && \
    /usr/local/lib/fpc/3.3.1/ppcrossarm -iV

#################################################
#                                               #
# Now build the usable cross compiler container #
#                                               #
#################################################

FROM ubuntu:bionic

ENV TERM=linux
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /usr/local/src

RUN apt-get update && \
    apt-get -y --no-install-recommends install \
        binutils-arm-none-eabi && \
    apt-get -y --purge autoremove && \
    apt-get -y upgrade && \
    apt-get clean && \
    rm -rf \
        /var/lib/apt/lists/* \
        /tmp/* \
        /var/tmp/* \
        /usr/src/doc/*

COPY --from=buildComp /usr/local/lib/fpc/3.3.1 /usr/local/lib/fpc/3.3.1
COPY --from=buildComp /usr/local/share/rtl.tar /usr/local/share/rtl.tar

RUN ln -s /usr/local/lib/fpc/3.3.1/ppcrossarm /usr/local/bin/fpc