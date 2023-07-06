
# Simulating Planetary Markets

The intended outcome of this is the simplest system that simulates prices based
on supply and demand in a way that appears organic to the player. The market trades
in generic, homogenous commodities such as Food or Clothes. Every market uses the
same currency.

In this initial vision, there are three categories of market members that interact
with the market in different ways:
- Producers - Members that can create commodities. They exist to drive supply for a
  given market.
- Consumers - Members that consume commodities. They exist to drive demand for a
  given market.
- Traders - Members that can purchase and sell commodities at a given market, and 
  can travel between markets, and therefore balancing out supply and demand.

## Desired behaviours:
- If the Producers in a market cannot meet the demand of the Consumers, prices
  should increase.
- If there is a surplus of supply in a Market prices should decrease.
- Traders should be able to exploit the above to make profit evening out supply
  and demand across different markets.

## Potential Solutions

There seem to be two classes of solutions, one where supply and demand is modelled
directly (Direct), and one where supply and demand is organic (Organic).

### Direct Modelling

In this approach the market can track supply and demand on a market and calculate
a market price for a given commodity (note, this is the current approach taken).
This approach seems simpler as there are no Buy Orders or Sell Orders. However some
aspects of the simulation become less obvious.

For example, if a Trader were to sell commodities in a Market, who do they sell
to? Does the Market have its own storage and wealth?

## Organic Modelling

In this approach commodity price when purchasing is defined as the lowest
existing Sell Order, and when selling the highest existing Buy Order. This
approach seems more natural but quickly brings up difficult questions concerning
the modelling.

For example, how long lived are Orders? Do they persist between ticks. In what order
do you create and execute the Orders?