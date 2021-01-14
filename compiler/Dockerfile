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

COPY fpu.patch /usr/local/src

RUN svn co http://svn.freepascal.org/svn/fpc/trunk fpc && \
    cd fpc && \
    rm -rf .svn && \
    patch -p1 -i ../fpu.patch && \
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

RUN ln -s /usr/local/lib/fpc/3.3.1/ppcrossarm /usr/local/bin/fpc && \
    ln -s /usr/bin/arm-none-eabi-addr2line /usr/bin/arm-embedded-addr2line && \
    ln -s /usr/bin/arm-none-eabi-ar /usr/bin/arm-embedded-ar && \
    ln -s /usr/bin/arm-none-eabi-as /usr/bin/arm-embedded-as && \
    ln -s /usr/bin/arm-none-eabi-c++filt /usr/bin/arm-embedded-c++filt && \
    ln -s /usr/bin/arm-none-eabi-elfedit /usr/bin/arm-embedded-elfedit && \
    ln -s /usr/bin/arm-none-eabi-gprof /usr/bin/arm-embedded-gprof && \
    ln -s /usr/bin/arm-none-eabi-ld /usr/bin/arm-embedded-ld && \
    ln -s /usr/bin/arm-none-eabi-ld.bfd /usr/bin/arm-embedded-ld.bfd && \
    ln -s /usr/bin/arm-none-eabi-nm /usr/bin/arm-embedded-nm && \
    ln -s /usr/bin/arm-none-eabi-objcopy /usr/bin/arm-embedded-objcopy && \
    ln -s /usr/bin/arm-none-eabi-objdump /usr/bin/arm-embedded-objdump && \
    ln -s /usr/bin/arm-none-eabi-ranlib /usr/bin/arm-embedded-ranlib && \
    ln -s /usr/bin/arm-none-eabi-readelf /usr/bin/arm-embedded-readelf && \
    ln -s /usr/bin/arm-none-eabi-size /usr/bin/arm-embedded-size && \
    ln -s /usr/bin/arm-none-eabi-strings /usr/bin/arm-embedded-strings && \
    ln -s /usr/bin/arm-none-eabi-strip /usr/bin/arm-embedded-strip

COPY --from=buildComp /usr/local/lib/fpc/3.3.1 /usr/local/lib/fpc/3.3.1
COPY fpc.cfg /usr/local/lib/fpc/etc/fpc.cfg

CMD ["bash"]
