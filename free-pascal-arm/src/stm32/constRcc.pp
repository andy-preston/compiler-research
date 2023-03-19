{* Based on C header file by STMicroelectronics
 *
 *******************************************************************************
 *
 * <h2><center>&copy; COPYRIGHT 2013 STMicroelectronics</center></h2>
 *
 * Licensed under MCD-ST Liberty SW License Agreement V2, (the "License");
 * You may not use this file except in compliance with the License.
 * You may obtain a copy of the License at:
 *
 *        http://www.st.com/software_license_agreement_liberty_v2
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 ******************************************************************************}

unit constRcc;

interface

type
    TAHB1Periph = (
        GPIOA =        $00000001,
        GPIOB =        $00000002,
        GPIOC =        $00000004,
        GPIOD =        $00000008,
        GPIOE =        $00000010,
        GPIOF =        $00000020,
        GPIOG =        $00000040,
        GPIOH =        $00000080,
        GPIOI =        $00000100,
        GPIOJ =        $00000200,
        GPIOK =        $00000400,
        CRC =          $00001000,
        FLITF =        $00008000,
        SRAM1 =        $00010000,
        SRAM2 =        $00020000,
        BKPSRAM =      $00040000,
        SRAM3 =        $00080000,
        CCMDATARAMEN = $00100000,
        DMA1 =         $00200000,
        DMA2 =         $00400000,
        DMA2D =        $00800000,
        ETH_MAC =      $02000000,
        ETH_MAC_Tx =   $04000000,
        ETH_MAC_Rx =   $08000000,
        ETH_MAC_PTP =  $10000000,
        OTG_HS =       $20000000,
        OTG_HS_ULPI =  $40000000
    );

implementation

end.
