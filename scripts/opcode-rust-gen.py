# usage: <script> [opcode_json_path]
import json
import sys

def num2hex(num):
    return '0x%02x' % num

def get_opcode_str(data):
    s = ''
    s += data['mnemonic']
    if 'operand1' in data:
        s += ' %s' % data['operand1']
    if 'operand2' in data:
        s += ',%s' % data['operand2']
    return s

def gen_table_mnemonic(data, table_name):
    print('pub const %s: [&\'static str; 256] = [' % table_name)
    for num in range(256):
        tmp = num2hex(num)
        nm  = ''
        if tmp in data:
            nm = get_opcode_str(data[tmp])
        print('\t\"%s\",' % nm)
    print('];')

def gen_table_ncycles(data, table_name):
    print('pub const %s: [(usize, usize); 256] = [' % table_name)
    for num in range(256):
        tmp = num2hex(num)

        c0 = 0
        c1 = 0
        nm = 'not an opcode'
        if tmp in data:
            c0 = data[tmp]['cycles'][0]
            c1 = data[tmp]['cycles'][1] if len(data[tmp]['cycles']) == 2 else c0
            nm = get_opcode_str(data[tmp])
        print('\t(%2d, %2d),  // %s, %s' % (c0, c1, tmp, nm))
    print('];')

def gen2(data, table_name, is_cb):
    print('pub const %s: [Option<Opcode>; 256] = [' % table_name)
    for num in range(256):
        tmp = num2hex(num)

        if tmp not in data:
            print('\tNone, // %s' % tmp)
            continue
        c0 = data[tmp]['cycles'][0]
        c1 = data[tmp]['cycles'][1] if len(data[tmp]['cycles']) == 2 else c0
        nm = get_opcode_str(data[tmp])
        is_cb_str = 'true' if is_cb else 'false'

        template = '\tSome( Opcode { '
        template += 'is_cb: %5s'
        template += ', value: %s'
        template += ', ncycles: (%2d, %2d)'
        template += ', mnemo: \"%s\"'
        template += ' } ),'
        print(template % (is_cb_str, tmp, c0, c1, nm))
    print('];')

def main():
    if len(sys.argv) != 2:
        print('usage: %s [opcode_path]' % sys.argv[0])
        exit(0)

    with open(sys.argv[1], 'r') as f:
        data = json.load(f)

        print('// Table that contains un-prefixed opcodes.')
        gen2(data['unprefixed'], 'TABLE_UN_PREFIXED', False)
        print('')

        print('// Table that contains cb-prefixed opcodes.')
        gen2(data['cbprefixed'], 'TABLE_CB_PREFIXED', True)
        print('')

        #print('// Table containing the mnemonics for un-prefixed opcodes')
        #gen_table_mnemonic(data['unprefixed'], 'TABLE_UN_PREFIXED_MNEMONICS')
        #print('')

        #print('// Table containing the ncycles for un-prefixed opcodes')
        #gen_table_ncycles(data['unprefixed'], 'TABLE_UN_PREFIXED_NCYCLES')
        #print('')

        #print('// Table containing the ncycles for cb-prefixed opcodes')
        #gen_table_ncycles(data['cbprefixed'], 'TABLE_CB_PREFIXED_NCYCLES')
        #print('')

        #print('// Table containing the mnemonics for cb-prefixed opcodes')
        #gen_table_mnemonic(data['cbprefixed'], 'TABLE_CB_PREFIXED_MNEMONICS')
        #print('')


if __name__ == '__main__':
    main()
