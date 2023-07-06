
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
- If there is a shotrage of supply in a Market prices should increase.
- If there is a surplus of supply in a Market prices should decrease.
- Traders (including the Player) should be able to exploit the above to make profit evening out supply
  and demand across different markets.

## Potential Solutions

There seem to be two classes of solutions, one where supply and demand is modelled
directly (Direct), and one where supply and demand is organic (Organic).

### Direct Modelling

In this approach the market tracks supply and demand on a market and artificially calculates a market price for a given
commodity. This is the current approach taken by the game as this approach seems simpler.

This approach requires that:
- Commodities have a fixed global base price.
- Markets have their own (practically infinite) commodity storage.

The simplest way to implement this is:
- Producers produce for the Market at their specified rate. 
- Consumers consume from the Market at their specified rate.
- Producers and Consumers have no Wealth.

This approach runs the risk of not seeming organic/reactive to the player. This could either be mitigated by migrating
to a more organic approach (see below), or by complicating the modeling.

For example, imagine if a planet produced commodity X, but nothing in the system consumed it. In the simplest
implementation that commodity would keep stockpiling, and the price would approach zero. Supply does not drive demand at
all, and vice versa.

## Organic Modelling

In this approach commodity price when purchasing is defined as the lowest
existing Sell Order, and when selling the highest existing Buy Order. This
approach seems more natural but quickly brings up difficult questions concerning
the modelling.

For example, how long-lived are Orders? Do they persist between ticks? If not what order
do you create and execute the Orders?