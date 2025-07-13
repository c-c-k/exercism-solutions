"""Solution to Ellen's Alien Game exercise."""


class Alien:
    """Create an Alien object with location x_coordinate and y_coordinate.

    Attributes
    ----------
    (class)total_aliens_created: int
    x_coordinate: int - Position on the x-axis.
    y_coordinate: int - Position on the y-axis.
    health: int - Number of health points.

    Methods
    -------
    hit(): Decrement Alien health by one point.
    is_alive(): Return a boolean for if Alien is alive (if health is > 0).
    teleport(new_x_coordinate, new_y_coordinate): Move Alien object to new coordinates.
    collision_detection(other): Implementation TBD.
    """

    total_aliens_created = 0
    x_coordinate: int
    y_coordinate: int
    health: int

    def __init__(self, x_coordinate: int, y_coordinate: int):
        self.x_coordinate = x_coordinate
        self.y_coordinate = y_coordinate
        self.health = 3
        Alien.total_aliens_created += 1

    def hit(self):
        """Decrement Alien health by one point."""
        self.health = self.health - 1 if self.is_alive else 0

    def is_alive(self):
        """Return a boolean for if Alien is alive (if health is > 0)."""
        return self.health > 0

    def teleport(self, new_x_coordinate: int, new_y_coordinate: int):
        """Move Alien object to new coordinates."""
        self.x_coordinate = new_x_coordinate
        self.y_coordinate = new_y_coordinate

    def collision_detection(self, other):
        """Implementation TBD."""
        pass


def new_aliens_collection(
    alien_start_positions: list[tuple[int, int]],
) -> list[Alien]:
    """Batch create aliens from coordinates.

    :param alien_start_positions: list of (X coordinate, Y coordinate)
    tuples representing aliens start positions.
    :return: list of `Alien` objects for the given start coordinates.

    """
    aliens = []
    for position in alien_start_positions:
        aliens.append(Alien(*position))
    return aliens
