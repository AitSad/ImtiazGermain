Installation
============

Requirements
------------

ImtiazGermain requires the following:

- Python 3.6+
- Sympy 
- Rust (no need for Python or Sympy)

Installation with pip
----------------------

ImtiazGermain can be installed using pip:

.. code-block:: bash

   $ pip install ImtiazGermain

Installation from source
------------------------

To install ImtiazGermain from source, follow these steps:

1. Clone the repository:

   .. code-block:: bash

      $ git clone https://github.com/AitSad/ImtiazGermain.git

2. Navigate to the repository directory:

   .. code-block:: bash

      $ cd ImtiazGermain

3. Install the package:

   .. code-block:: bash

      $ pip install .

   This will install the package and its dependencies.

   Alternatively, you can install the package in development mode:

   .. code-block:: bash

      $ pip install -e .

   This will install the package in editable mode, so changes you make to the code will be immediately reflected.

   Note: You may need to use `sudo` to install the package globally on your system.


Installation from cargo
------------------------

You can use the cargo module in rust, to load ImtiazGermain as a crate as following:

.. code-block:: toml

    [dependencies]
    ImtiazGermain = "0.1.1"

Upgrade
-------

To upgrade ImtiazGermain to the latest version, use pip:

.. code-block:: bash

   $ pip install --upgrade ImtiazGermain

