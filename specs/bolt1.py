# code generated by the lncodegen.rs please do not edit

from lnspec_py.basic_type.int import U16Int, U32Int
from lnspec_py.basic_type.hex_type import ChannelId
from lnspec_py.basic_type.bitmask import Bitfield
from lnspec_py.basic_type.tvl_record import TVLRecord
    # close scope

class ErrorMsg: 

    def __init__(self, msg_type, channel_id, len, data):
            self.msg_type = msg_type
            self.channel_id = channel_id
            self.len = len
            self.data = data
        # close scope


    def encode(self) -> str:
            raw_msg = ''
            raw_msg += '{}'.format(self.msg_type.encode())raw_msg += selfchannel_id.encode()
            raw_msg += '{}'.format(self.len.encode())
            if len(self.data.bitfield) > 0:
             	 raw_msg += Bitfield.encode(self.data.bitfield)
            return raw_msg
        # close scope
    
    
        @staticmethod
        def decode(raw_msg: str):
            msg_type, raw_msg = U16Int.decode_with_hex_str(raw_msg)channel_id, raw_msg = ChannelId.decode_from_hex(raw_msg)
            len, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            data, raw_msg = Bitfield.decode_with_len(raw_msg)
            return ErrorMsg(msg_type, channel_id, len, data)
    # close scope
# close scope


    # close scope

class InitMsg: 

    def __init__(self, msg_type, gflen, globalfeatures, flen, features, init_tlvs):
            self.msg_type = msg_type
            self.gflen = gflen
            self.globalfeatures = globalfeatures
            self.flen = flen
            self.features = features
            self.init_tlvs = init_tlvs
        # close scope


    def encode(self) -> str:
            raw_msg = ''
            raw_msg += '{}'.format(self.msg_type.encode())
            raw_msg += '{}'.format(self.gflen.encode())
            if len(self.globalfeatures.bitfield) > 0:
             	 raw_msg += Bitfield.encode(self.globalfeatures.bitfield)
            raw_msg += '{}'.format(self.flen.encode())
            if len(self.features.bitfield) > 0:
             	 raw_msg += Bitfield.encode(self.features.bitfield)
            raw_msg += self.init_tlvs.encode()
            return raw_msg
        # close scope
    
    
        @staticmethod
        def decode(raw_msg: str):
            msg_type, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            gflen, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            globalfeatures, raw_msg = Bitfield.decode_with_len(raw_msg)
            flen, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            features, raw_msg = Bitfield.decode_with_len(raw_msg)
            init_tlvs = TVLRecord(raw_msg)
            init_tlvs.decode()
            return InitMsg(msg_type, gflen, globalfeatures, flen, features, init_tlvs)
    # close scope
# close scope


    # close scope

class PingMsg: 

    def __init__(self, msg_type, num_pong_bytes, byteslen, ignored):
            self.msg_type = msg_type
            self.num_pong_bytes = num_pong_bytes
            self.byteslen = byteslen
            self.ignored = ignored
        # close scope


    def encode(self) -> str:
            raw_msg = ''
            raw_msg += '{}'.format(self.msg_type.encode())
            raw_msg += '{}'.format(self.num_pong_bytes.encode())
            raw_msg += '{}'.format(self.byteslen.encode())
            if len(self.ignored.bitfield) > 0:
             	 raw_msg += Bitfield.encode(self.ignored.bitfield)
            return raw_msg
        # close scope
    
    
        @staticmethod
        def decode(raw_msg: str):
            msg_type, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            num_pong_bytes, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            byteslen, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            ignored, raw_msg = Bitfield.decode_with_len(raw_msg)
            return PingMsg(msg_type, num_pong_bytes, byteslen, ignored)
    # close scope
# close scope


    # close scope

class PongMsg: 

    def __init__(self, msg_type, byteslen, ignored):
            self.msg_type = msg_type
            self.byteslen = byteslen
            self.ignored = ignored
        # close scope


    def encode(self) -> str:
            raw_msg = ''
            raw_msg += '{}'.format(self.msg_type.encode())
            raw_msg += '{}'.format(self.byteslen.encode())
            if len(self.ignored.bitfield) > 0:
             	 raw_msg += Bitfield.encode(self.ignored.bitfield)
            return raw_msg
        # close scope
    
    
        @staticmethod
        def decode(raw_msg: str):
            msg_type, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            byteslen, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            ignored, raw_msg = Bitfield.decode_with_len(raw_msg)
            return PongMsg(msg_type, byteslen, ignored)
    # close scope
# close scope


    # close scope

class WarningMsg: 

    def __init__(self, msg_type, channel_id, len, data):
            self.msg_type = msg_type
            self.channel_id = channel_id
            self.len = len
            self.data = data
        # close scope


    def encode(self) -> str:
            raw_msg = ''
            raw_msg += '{}'.format(self.msg_type.encode())raw_msg += selfchannel_id.encode()
            raw_msg += '{}'.format(self.len.encode())
            if len(self.data.bitfield) > 0:
             	 raw_msg += Bitfield.encode(self.data.bitfield)
            return raw_msg
        # close scope
    
    
        @staticmethod
        def decode(raw_msg: str):
            msg_type, raw_msg = U16Int.decode_with_hex_str(raw_msg)channel_id, raw_msg = ChannelId.decode_from_hex(raw_msg)
            len, raw_msg = U16Int.decode_with_hex_str(raw_msg)
            data, raw_msg = Bitfield.decode_with_len(raw_msg)
            return WarningMsg(msg_type, channel_id, len, data)
    # close scope
# close scope

