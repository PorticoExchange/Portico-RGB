# Portico-RGB

RGB&lt;>Portico client

# How works?

Simulation A

In this example, we define an HTLC with a preimage of all 0x01 bytes, a hash of the preimage, a value of 1000 satoshis, and a timelock of 100 blocks. We then create an HTLC input and output using the HTLC Output struct provided by RGB Core, with the HTLC_SCRIPT script provided by the rgb::schema::scripts module. Finally, we create RGB Core input and output entries using the Entry struct provided by RGB Core, and encode the HTLC output as a Vec<u8> using the serialize method provided by the Serialize trait implemented for the HTLCOutput struct.

Note that this code only creates the input and output for an HTLC, and does not include any logic for executing the HTLC

Simulation B
  
In this example, we define contract on RGB and allowing input/output inside transaction without need HTLC
